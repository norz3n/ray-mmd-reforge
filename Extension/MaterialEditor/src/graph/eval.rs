//! Graph evaluator that compiles nodes into baked texture maps.

use std::collections::HashMap;
use egui_snarl::{InPinId, NodeId, Snarl};
use crate::graph::node::{BlendMode, MaterialNode};
use crate::image_proc::*;

/// Evaluated material maps ready for 3D preview and Ray-MMD export.
#[derive(Default, Clone)]
pub struct EvaluatedMaterial {
    pub albedo: Option<U8Image>,
    pub albedo_sub: Option<U8Image>,
    pub alpha: Option<U8Image>,
    pub normal: Option<U8Image>,
    pub smoothness: Option<U8Image>,
    pub metalness: Option<U8Image>,
    pub specular: Option<U8Image>,
    pub occlusion: Option<U8Image>,
    pub parallax: Option<U8Image>,
    pub emissive: Option<U8Image>,
    pub custom_a: Option<U8Image>,
    pub custom_b: Option<U8Image>,
    pub detail_normal: Option<U8Image>,
}

/// Helper to create a solid 1x1 color image.
pub fn create_solid_color_image(rgba: [u8; 4], width: u32, height: u32) -> U8Image {
    let mut img = U8Image::new(width.max(1), height.max(1));
    for p in img.pixels_mut() {
        *p = image::Rgba(rgba);
    }
    img
}

/// Context for evaluating nodes in the graph.
pub struct GraphEvaluator<'a> {
    snarl: &'a Snarl<MaterialNode>,
    cache: HashMap<(NodeId, usize), U8Image>,
    eval_stack: Vec<NodeId>, // cycle detection
    pub working_resolution: u32,
}

impl<'a> GraphEvaluator<'a> {
    pub fn new(snarl: &'a Snarl<MaterialNode>) -> Self {
        Self::with_resolution(snarl, 512)
    }

    pub fn with_resolution(snarl: &'a Snarl<MaterialNode>, working_resolution: u32) -> Self {
        Self {
            snarl,
            cache: HashMap::new(),
            eval_stack: Vec::new(),
            working_resolution,
        }
    }

    #[inline(always)]
    pub fn default_dim(&self) -> u32 {
        if self.working_resolution > 0 {
            self.working_resolution.min(512).max(64)
        } else {
            256
        }
    }

    /// Evaluates the input connected to `node` at `input_index`.
    pub fn eval_input(&mut self, node_id: NodeId, input_index: usize) -> Option<U8Image> {
        let in_pin = self.snarl.in_pin(InPinId {
            node: node_id,
            input: input_index,
        });

        if let Some(remote) = in_pin.remotes.first() {
            self.eval_node_output(remote.node, remote.output)
        } else {
            None
        }
    }

    /// Recursively evaluates a node's output pin.
    pub fn eval_node_output(&mut self, node_id: NodeId, output_index: usize) -> Option<U8Image> {
        let key = (node_id, output_index);
        if let Some(cached) = self.cache.get(&key) {
            return Some(cached.clone());
        }

        // Cycle detection
        if self.eval_stack.contains(&node_id) {
            return None;
        }
        self.eval_stack.push(node_id);

        let dim = self.default_dim();
        let node = self.snarl.get_node(node_id)?;
        let result = match node {
            MaterialNode::ImageInput {
                file_path,
                cached_image,
                ..
            } => {
                let base = if let Some(img) = cached_image {
                    Some(img.clone())
                } else if !file_path.is_empty() {
                    match image::open(file_path) {
                        Ok(dyn_img) => Some(dyn_img.to_rgba8()),
                        Err(_) => Some(create_solid_color_image([255, 255, 255, 255], dim, dim)),
                    }
                } else {
                    Some(create_solid_color_image([255, 255, 255, 255], dim, dim))
                };

                if let Some(img) = base {
                    if self.working_resolution > 0 && (img.width() > self.working_resolution || img.height() > self.working_resolution) {
                        let scale = (self.working_resolution as f32 / img.width().max(img.height()) as f32).min(1.0);
                        let nw = ((img.width() as f32 * scale).round() as u32).max(1);
                        let nh = ((img.height() as f32 * scale).round() as u32).max(1);
                        Some(image::imageops::resize(&img, nw, nh, image::imageops::FilterType::Triangle))
                    } else {
                        Some(img)
                    }
                } else {
                    None
                }
            }
            MaterialNode::ColorInput { color } => {
                let rgba = [
                    (color[0].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                    (color[1].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                    (color[2].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                    (color[3].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                ];
                Some(create_solid_color_image(rgba, dim, dim))
            }
            MaterialNode::FloatInput { value, .. } => {
                let byte = (value.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                Some(create_solid_color_image([byte, byte, byte, 255], dim, dim))
            }
            MaterialNode::HeightGenerator {
                contrast,
                brightness,
                invert,
            } => {
                let input_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                Some(generate_height_map(&input_img, *contrast, *brightness, *invert))
            }
            MaterialNode::NormalGenerator {
                scale,
                filter,
                orientation,
            } => {
                let input_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                Some(generate_normal_map(&input_img, *scale, *filter, *orientation))
            }
            MaterialNode::AOGenerator {
                radius,
                samples,
                intensity,
                bias,
            } => {
                let input_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                Some(generate_ao_map(&input_img, *radius, *samples, *intensity, *bias))
            }
            MaterialNode::CurvatureGenerator {
                radius,
                intensity,
                mode,
            } => {
                let input_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                Some(generate_curvature_map(&input_img, *radius, *intensity, *mode))
            }
            MaterialNode::RoughnessGenerator {
                invert,
                contrast,
                min_val,
                max_val,
            } => {
                let input_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                Some(generate_roughness_map(
                    &input_img, *invert, *contrast, *min_val, *max_val,
                ))
            }
            MaterialNode::NormalBlend {
                detail_scale,
                detail_tile,
            } => {
                let base_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 255, 255], dim, dim));
                let detail_img = self
                    .eval_input(node_id, 1)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 255, 255], dim, dim));
                Some(blend_normals_rnm(
                    &base_img,
                    &detail_img,
                    *detail_scale,
                    *detail_tile,
                ))
            }
            MaterialNode::ChannelPacker {
                default_r,
                default_g,
                default_b,
                default_a,
            } => {
                let r_img = self.eval_input(node_id, 0);
                let g_img = self.eval_input(node_id, 1);
                let b_img = self.eval_input(node_id, 2);
                let a_img = self.eval_input(node_id, 3);
                Some(pack_channels(
                    r_img.as_ref(),
                    g_img.as_ref(),
                    b_img.as_ref(),
                    a_img.as_ref(),
                    [*default_r, *default_g, *default_b, *default_a],
                ))
            }
            MaterialNode::ChannelSplitter => {
                let in_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([128, 128, 128, 255], dim, dim));
                let (w, h) = in_img.dimensions();
                let mut out = U8Image::new(w, h);
                let ch_idx = output_index.min(3);
                for (x, y, pixel) in in_img.enumerate_pixels() {
                    let val = pixel[ch_idx];
                    out.put_pixel(x, y, image::Rgba([val, val, val, 255]));
                }
                Some(out)
            }
            MaterialNode::ColorBlend { mode, factor } => {
                let base = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([255, 255, 255, 255], dim, dim));
                let blend = self
                    .eval_input(node_id, 1)
                    .unwrap_or_else(|| create_solid_color_image([255, 255, 255, 255], dim, dim));

                let (w, h) = (base.width().max(blend.width()), base.height().max(blend.height()));
                let mut out = U8Image::new(w, h);
                let f = factor.clamp(0.0, 1.0);

                for y in 0..h {
                    for x in 0..w {
                        let bx = (x * base.width() / w).min(base.width() - 1);
                        let by = (y * base.height() / h).min(base.height() - 1);
                        let lx = (x * blend.width() / w).min(blend.width() - 1);
                        let ly = (y * blend.height() / h).min(blend.height() - 1);

                        let b_pix = base.get_pixel(bx, by);
                        let l_pix = blend.get_pixel(lx, ly);

                        let mut result = [0u8; 4];
                        for c in 0..3 {
                            let b_val = b_pix[c] as f32 / 255.0;
                            let l_val = l_pix[c] as f32 / 255.0;

                            let blended = match mode {
                                BlendMode::Mix => l_val,
                                BlendMode::Multiply => b_val * l_val,
                                BlendMode::Screen => 1.0 - (1.0 - b_val) * (1.0 - l_val),
                                BlendMode::Overlay => {
                                    if b_val < 0.5 {
                                        2.0 * b_val * l_val
                                    } else {
                                        1.0 - 2.0 * (1.0 - b_val) * (1.0 - l_val)
                                    }
                                }
                                BlendMode::Add => (b_val + l_val).min(1.0),
                                BlendMode::Subtract => (b_val - l_val).max(0.0),
                            };
                            let final_val = b_val * (1.0 - f) + blended * f;
                            result[c] = (final_val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        }
                        result[3] = b_pix[3];
                        out.put_pixel(x, y, image::Rgba(result));
                    }
                }
                Some(out)
            }
            MaterialNode::MetalnessGenerator {
                threshold,
                falloff,
                detect_metals,
                invert,
            } => {
                let base_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([180, 180, 180, 255], dim, dim));
                Some(generate_metalness_map(
                    &base_img,
                    *threshold,
                    *falloff,
                    *detect_metals,
                    *invert,
                ))
            }
            MaterialNode::EmissiveGenerator {
                min_lum,
                max_lum,
                use_hue_filter,
                target_hue,
                hue_tolerance,
                tint_color,
                intensity,
                invert,
            } => {
                let base_img = self
                    .eval_input(node_id, 0)
                    .unwrap_or_else(|| create_solid_color_image([255, 255, 255, 255], dim, dim));
                let mask_img = self.eval_input(node_id, 1);
                let target_hue_opt = if *use_hue_filter { Some(*target_hue) } else { None };
                Some(generate_advanced_emissive_mask(
                    &base_img,
                    *min_lum,
                    *max_lum,
                    target_hue_opt,
                    *hue_tolerance,
                    *tint_color,
                    *intensity,
                    *invert,
                    mask_img.as_ref(),
                ))
            }
            MaterialNode::CustomMapGenerator {
                model,
                param_a,
                param_b_color,
                invert_a,
                aniso_radial,
            } => {
                if output_index == 0 {
                    // Custom A (Grayscale)
                    if let Some(guide) = self.eval_input(node_id, 0) {
                        let (w, h) = guide.dimensions();
                        let mut out = U8Image::new(w, h);
                        for (x, y, pix) in guide.enumerate_pixels() {
                            let g = pix[0] as f32 / 255.0;
                            let val = if *invert_a { (1.0 - g) * param_a } else { g * param_a };
                            let b = (val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                            out.put_pixel(x, y, image::Rgba([b, b, b, 255]));
                        }
                        Some(out)
                    } else {
                        let val = if *invert_a { 1.0 - param_a } else { *param_a };
                        let b = (val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        Some(create_solid_color_image([b, b, b, 255], dim, dim))
                    }
                } else {
                    // Custom B (RGBA / Color)
                    match model {
                        crate::graph::node::ShadingModel::Anisotropy => {
                            Some(crate::image_proc::generate_anisotropy_tangent_map(
                                dim,
                                dim,
                                param_a * std::f32::consts::TAU,
                                *aniso_radial,
                            ))
                        }
                        _ => {
                            if let Some(guide) = self.eval_input(node_id, 1) {
                                let (w, h) = guide.dimensions();
                                let mut out = U8Image::new(w, h);
                                for (x, y, pix) in guide.enumerate_pixels() {
                                    let r = (pix[0] as f32 / 255.0 * param_b_color[0]).clamp(0.0, 1.0);
                                    let g = (pix[1] as f32 / 255.0 * param_b_color[1]).clamp(0.0, 1.0);
                                    let b = (pix[2] as f32 / 255.0 * param_b_color[2]).clamp(0.0, 1.0);
                                    out.put_pixel(
                                        x,
                                        y,
                                        image::Rgba([
                                            (r * 255.0 + 0.5) as u8,
                                            (g * 255.0 + 0.5) as u8,
                                            (b * 255.0 + 0.5) as u8,
                                            255,
                                        ]),
                                    );
                                }
                                Some(out)
                            } else {
                                let rgba = [
                                    (param_b_color[0].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                                    (param_b_color[1].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                                    (param_b_color[2].clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
                                    255,
                                ];
                                Some(create_solid_color_image(rgba, dim, dim))
                            }
                        }
                    }
                }
            }
            MaterialNode::RayMaterialOutput { .. } => None,
        };

        self.eval_stack.pop();

        if let Some(ref img) = result {
            self.cache.insert(key, img.clone());
        }

        result
    }

    /// Evaluates the entire graph into the master `EvaluatedMaterial` channels.
    pub fn evaluate_material(&mut self) -> EvaluatedMaterial {
        let mut mat = EvaluatedMaterial::default();

        // Find the RayMaterialOutput node
        let mut master_node_id = None;
        for (id, node) in self.snarl.node_ids() {
            if matches!(node, MaterialNode::RayMaterialOutput { .. }) {
                master_node_id = Some(id);
                break;
            }
        }

        let output_id = match master_node_id {
            Some(id) => id,
            None => return mat,
        };

        mat.albedo = self.eval_input(output_id, 0);
        mat.albedo_sub = self.eval_input(output_id, 1);
        mat.alpha = self.eval_input(output_id, 2);
        mat.normal = self.eval_input(output_id, 3);
        mat.smoothness = self.eval_input(output_id, 4);
        mat.metalness = self.eval_input(output_id, 5);
        mat.specular = self.eval_input(output_id, 6);
        mat.occlusion = self.eval_input(output_id, 7);
        mat.parallax = self.eval_input(output_id, 8);
        mat.emissive = self.eval_input(output_id, 9);
        mat.custom_a = self.eval_input(output_id, 10);
        mat.custom_b = self.eval_input(output_id, 11);
        mat.detail_normal = self.eval_input(output_id, 12);

        // Fallback: if channels are not connected to output pins, still evaluate any generator nodes in graph
        if mat.metalness.is_none() {
            for (id, node) in self.snarl.node_ids() {
                if matches!(node, MaterialNode::MetalnessGenerator { .. }) {
                    mat.metalness = self.eval_node_output(id, 0);
                    break;
                }
            }
        }
        if mat.emissive.is_none() {
            for (id, node) in self.snarl.node_ids() {
                if matches!(node, MaterialNode::EmissiveGenerator { .. }) {
                    mat.emissive = self.eval_node_output(id, 0);
                    break;
                }
            }
        }
        if mat.custom_a.is_none() {
            for (id, node) in self.snarl.node_ids() {
                if matches!(node, MaterialNode::CustomMapGenerator { .. }) {
                    mat.custom_a = self.eval_node_output(id, 0);
                    if mat.custom_b.is_none() {
                        mat.custom_b = self.eval_node_output(id, 1);
                    }
                    break;
                }
            }
        }

        mat
    }
}
