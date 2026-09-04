//! Snarl viewer implementation for drawing and interacting with material nodes in egui.

use egui::{Color32, DragValue, Slider, Ui};
use egui_snarl::ui::{PinInfo, PinShape, SnarlPin, SnarlViewer};
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use crate::graph::node::{BlendMode, MaterialNode, PinType};
use crate::image_proc::{CurvatureMode, NormalFilter, NormalOrientation};

/// Returns pin styling and shape based on data type.
pub fn pin_info_for_type(pin_type: PinType) -> PinInfo {
    match pin_type {
        PinType::Rgba => PinInfo::circle()
            .with_shape(PinShape::Circle)
            .with_fill(Color32::from_rgb(240, 180, 50))
            .with_wire_color(Color32::from_rgb(240, 180, 50)),
        PinType::Grayscale => PinInfo::circle()
            .with_shape(PinShape::Circle)
            .with_fill(Color32::from_rgb(180, 180, 190))
            .with_wire_color(Color32::from_rgb(180, 180, 190)),
        PinType::Float => PinInfo::circle()
            .with_shape(PinShape::Triangle)
            .with_fill(Color32::from_rgb(60, 160, 240))
            .with_wire_color(Color32::from_rgb(60, 160, 240)),
    }
}

/// The viewer state for egui-snarl.
pub struct MaterialSnarlViewer {
    /// Flag indicating the graph was modified and requires re-evaluation.
    pub needs_rebuild: bool,
    /// List of nodes queued for deletion after frame rendering.
    pub nodes_to_remove: Vec<NodeId>,
}

impl MaterialSnarlViewer {
    pub fn new() -> Self {
        Self {
            needs_rebuild: true,
            nodes_to_remove: Vec::new(),
        }
    }
}

impl SnarlViewer<MaterialNode> for MaterialSnarlViewer {
    fn title(&mut self, node: &MaterialNode) -> String {
        node.title().to_string()
    }

    fn show_header(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.title(&snarl[node])).strong());
            let is_master = matches!(snarl[node], MaterialNode::RayMaterialOutput { .. });
            if !is_master {
                if ui.button(egui::RichText::new("✖").color(Color32::from_rgb(255, 90, 90))).on_hover_text("Delete node").clicked() {
                    self.nodes_to_remove.push(node);
                    self.needs_rebuild = true;
                }
            }
        });
    }

    fn has_node_menu(&mut self, _node: &MaterialNode) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) {
        let is_master = matches!(snarl[node], MaterialNode::RayMaterialOutput { .. });
        if !is_master {
            if ui.button("🗑 Delete Node").clicked() {
                self.nodes_to_remove.push(node);
                self.needs_rebuild = true;
                ui.close();
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<MaterialNode>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) {
        ui.label(egui::RichText::new("Create Node:").strong());
        ui.separator();

        if ui.button("➕ Texture Input").clicked() {
            snarl.insert_node(pos, MaterialNode::ImageInput {
                file_path: String::new(),
                is_srgb: true,
                cached_image: None,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("➕ Color Value").clicked() {
            snarl.insert_node(pos, MaterialNode::ColorInput { color: [1.0, 1.0, 1.0, 1.0] });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("➕ Float Value").clicked() {
            snarl.insert_node(pos, MaterialNode::FloatInput { value: 1.0, min: 0.0, max: 1.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        ui.separator();
        if ui.button("⚡ Height Generator").clicked() {
            snarl.insert_node(pos, MaterialNode::HeightGenerator { contrast: 1.0, brightness: 0.0, invert: false });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("⚡ Normal Generator").clicked() {
            snarl.insert_node(pos, MaterialNode::NormalGenerator { scale: 1.0, filter: NormalFilter::Scharr, orientation: NormalOrientation::DirectX });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("⚡ Ambient Occlusion (AO)").clicked() {
            snarl.insert_node(pos, MaterialNode::AOGenerator { radius: 16, samples: 16, intensity: 1.0, bias: 0.05 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("⚡ Curvature / Cavity").clicked() {
            snarl.insert_node(pos, MaterialNode::CurvatureGenerator { radius: 2, intensity: 2.0, mode: CurvatureMode::Full });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("⚡ Roughness Remap").clicked() {
            snarl.insert_node(pos, MaterialNode::RoughnessGenerator { invert: false, contrast: 1.0, min_val: 0.0, max_val: 1.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("🪙 Metalness Generator").clicked() {
            snarl.insert_node(pos, MaterialNode::MetalnessGenerator {
                threshold: 0.5,
                falloff: 0.2,
                detect_metals: true,
                invert: false,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("💡 Emissive Generator").clicked() {
            snarl.insert_node(pos, MaterialNode::EmissiveGenerator {
                min_lum: 0.5,
                max_lum: 1.0,
                use_hue_filter: false,
                target_hue: 180.0,
                hue_tolerance: 45.0,
                tint_color: [1.0, 1.0, 1.0],
                intensity: 2.0,
                invert: false,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("🎭 Custom Map Generator").clicked() {
            snarl.insert_node(pos, MaterialNode::CustomMapGenerator {
                model: crate::graph::node::ShadingModel::Skin,
                param_a: 1.0,
                param_b_color: [1.0, 0.4, 0.25],
                invert_a: false,
                aniso_radial: false,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        ui.separator();
        if ui.button("🔀 Normal Blend (RNM)").clicked() {
            snarl.insert_node(pos, MaterialNode::NormalBlend { detail_scale: 1.0, detail_tile: 10.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("📦 Channel Packer (RGBA)").clicked() {
            snarl.insert_node(pos, MaterialNode::ChannelPacker { default_r: 128, default_g: 0, default_b: 255, default_a: 255 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("✂ Channel Splitter").clicked() {
            snarl.insert_node(pos, MaterialNode::ChannelSplitter);
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("🎨 Color Blend").clicked() {
            snarl.insert_node(pos, MaterialNode::ColorBlend { mode: BlendMode::Mix, factor: 0.5 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button("⚡ Procedural Noise").clicked() {
            snarl.insert_node(pos, MaterialNode::ProceduralNoise {
                noise_type: crate::image_proc::NoiseType::Perlin,
                scale: 4.0,
                octaves: 4,
                lacunarity: 2.0,
                gain: 0.5,
            });
            self.needs_rebuild = true;
            ui.close();
        }
    }

    fn inputs(&mut self, node: &MaterialNode) -> usize {
        node.input_count()
    }

    fn outputs(&mut self, node: &MaterialNode) -> usize {
        node.output_count()
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<MaterialNode>) {
        snarl.connect(from.id, to.id);
        self.needs_rebuild = true;
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<MaterialNode>) {
        snarl.disconnect(from.id, to.id);
        self.needs_rebuild = true;
    }

    fn drop_outputs(&mut self, pin: &OutPin, snarl: &mut Snarl<MaterialNode>) {
        snarl.drop_outputs(pin.id);
        self.needs_rebuild = true;
    }

    fn drop_inputs(&mut self, pin: &InPin, snarl: &mut Snarl<MaterialNode>) {
        snarl.drop_inputs(pin.id);
        self.needs_rebuild = true;
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let (name, pin_type) = node.input_info(pin.id.input);
        ui.label(name);
        pin_info_for_type(pin_type)
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let (name, pin_type) = node.output_info(pin.id.output);
        ui.label(name);
        pin_info_for_type(pin_type)
    }

    fn has_body(&mut self, _node: &MaterialNode) -> bool {
        true
    }

    fn show_body(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<MaterialNode>,
    ) {
        let node = &mut snarl[node_id];
        let mut changed = false;

        match node {
            MaterialNode::ImageInput {
                file_path,
                is_srgb,
                cached_image,
            } => {
                ui.horizontal(|ui| {
                    if ui.button("📁 Open File...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "tga", "dds", "bmp"])
                            .pick_file()
                        {
                            *file_path = path.to_string_lossy().to_string();
                            if let Ok(dyn_img) = image::open(&path) {
                                *cached_image = Some(dyn_img.to_rgba8());
                            }
                            changed = true;
                        }
                    }
                    if !file_path.is_empty() {
                        let short_name = std::path::Path::new(file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(file_path);
                        ui.label(short_name);
                    }
                });
                changed |= ui.checkbox(is_srgb, "sRGB Color Space").changed();
            }
            MaterialNode::ColorInput { color } => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    changed |= ui.color_edit_button_rgba_unmultiplied(color).changed();
                });
            }
            MaterialNode::FloatInput { value, min, max } => {
                changed |= ui.add(Slider::new(value, *min..=*max).step_by(0.01)).changed();
            }
            MaterialNode::HeightGenerator {
                contrast,
                brightness,
                invert,
            } => {
                changed |= ui.add(Slider::new(contrast, 0.0..=5.0).text("Contrast")).changed();
                changed |= ui.add(Slider::new(brightness, -1.0..=1.0).text("Brightness")).changed();
                changed |= ui.checkbox(invert, "Invert Height").changed();
            }
            MaterialNode::NormalGenerator {
                scale,
                filter,
                orientation,
            } => {
                changed |= ui.add(Slider::new(scale, 0.05..=10.0).text("Bump Scale")).changed();
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    egui::ComboBox::from_id_salt((node_id, "filt"))
                        .selected_text(match filter {
                            NormalFilter::Sobel => "Sobel 3x3",
                            NormalFilter::Scharr => "Scharr 3x3",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(filter, NormalFilter::Sobel, "Sobel 3x3").changed();
                            changed |= ui.selectable_value(filter, NormalFilter::Scharr, "Scharr 3x3").changed();
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    egui::ComboBox::from_id_salt((node_id, "orient"))
                        .selected_text(match orientation {
                            NormalOrientation::DirectX => "DirectX (Y- Down)",
                            NormalOrientation::OpenGL => "OpenGL (Y+ Up)",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(orientation, NormalOrientation::DirectX, "DirectX (Y- Down)").changed();
                            changed |= ui.selectable_value(orientation, NormalOrientation::OpenGL, "OpenGL (Y+ Up)").changed();
                        });
                });
            }
            MaterialNode::AOGenerator {
                radius,
                samples,
                intensity,
                bias,
            } => {
                changed |= ui.add(Slider::new(radius, 2..=64).text("Radius (px)")).changed();
                changed |= ui.add(Slider::new(samples, 4..=32).text("Ray Directions")).changed();
                changed |= ui.add(Slider::new(intensity, 0.0..=5.0).text("Intensity")).changed();
                changed |= ui.add(Slider::new(bias, -0.2..=0.5).text("Horizon Bias")).changed();
            }
            MaterialNode::CurvatureGenerator {
                radius,
                intensity,
                mode,
            } => {
                changed |= ui.add(Slider::new(radius, 1..=16).text("Sample Radius")).changed();
                changed |= ui.add(Slider::new(intensity, 0.1..=10.0).text("Intensity")).changed();
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    egui::ComboBox::from_id_salt((node_id, "curv_mode"))
                        .selected_text(match mode {
                            CurvatureMode::Full => "Full (0.5 flat)",
                            CurvatureMode::ConvexOnly => "Convex (Ridges)",
                            CurvatureMode::ConcaveOnly => "Concave (Cavities)",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(mode, CurvatureMode::Full, "Full (0.5 flat)").changed();
                            changed |= ui.selectable_value(mode, CurvatureMode::ConvexOnly, "Convex (Ridges)").changed();
                            changed |= ui.selectable_value(mode, CurvatureMode::ConcaveOnly, "Concave (Cavities)").changed();
                        });
                });
            }
            MaterialNode::RoughnessGenerator {
                invert,
                contrast,
                min_val,
                max_val,
            } => {
                changed |= ui.checkbox(invert, "Invert (Gloss <-> Rough)").changed();
                changed |= ui.add(Slider::new(contrast, 0.1..=5.0).text("Contrast")).changed();
                changed |= ui.add(Slider::new(min_val, 0.0..=1.0).text("Min Output")).changed();
                changed |= ui.add(Slider::new(max_val, 0.0..=1.0).text("Max Output")).changed();
            }
            MaterialNode::NormalBlend {
                detail_scale,
                detail_tile,
            } => {
                changed |= ui.add(Slider::new(detail_scale, 0.0..=5.0).text("Detail Scale")).changed();
                changed |= ui.add(Slider::new(detail_tile, 1.0..=50.0).text("Detail Repeat")).changed();
            }
            MaterialNode::ChannelPacker {
                default_r,
                default_g,
                default_b,
                default_a,
            } => {
                ui.label("Default values if unplugged:");
                ui.horizontal(|ui| {
                    ui.label("R:");
                    changed |= ui.add(DragValue::new(default_r).range(0..=255)).changed();
                    ui.label("G:");
                    changed |= ui.add(DragValue::new(default_g).range(0..=255)).changed();
                    ui.label("B:");
                    changed |= ui.add(DragValue::new(default_b).range(0..=255)).changed();
                    ui.label("A:");
                    changed |= ui.add(DragValue::new(default_a).range(0..=255)).changed();
                });
            }
            MaterialNode::ChannelSplitter => {
                ui.label("Splits RGBA into individual channels.");
            }
            MaterialNode::ColorBlend { mode, factor } => {
                ui.horizontal(|ui| {
                    ui.label("Blend Mode:");
                    egui::ComboBox::from_id_salt((node_id, "blend_mode"))
                        .selected_text(match mode {
                            BlendMode::Mix => "Mix / Linear",
                            BlendMode::Multiply => "Multiply",
                            BlendMode::Screen => "Screen",
                            BlendMode::Overlay => "Overlay",
                            BlendMode::Add => "Add",
                            BlendMode::Subtract => "Subtract",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(mode, BlendMode::Mix, "Mix / Linear").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Multiply, "Multiply").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Screen, "Screen").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Overlay, "Overlay").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Add, "Add").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Subtract, "Subtract").changed();
                        });
                });
                changed |= ui.add(Slider::new(factor, 0.0..=1.0).text("Blend Factor")).changed();
            }
            MaterialNode::ProceduralNoise {
                noise_type,
                scale,
                octaves,
                lacunarity,
                gain,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Noise Type:");
                    egui::ComboBox::from_id_salt((node_id, "noise_type_sel"))
                        .selected_text(match noise_type {
                            crate::image_proc::NoiseType::Perlin => "Perlin fBm",
                            crate::image_proc::NoiseType::Voronoi => "Voronoi Cellular",
                            crate::image_proc::NoiseType::WhiteNoise => "White Noise",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::Perlin, "Perlin fBm").changed();
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::Voronoi, "Voronoi Cellular").changed();
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::WhiteNoise, "White Noise").changed();
                        });
                });
                changed |= ui.add(Slider::new(scale, 0.1..=50.0).text("Scale / Frequency")).changed();
                if *noise_type == crate::image_proc::NoiseType::Perlin {
                    changed |= ui.add(Slider::new(octaves, 1..=8).text("Octaves")).changed();
                    changed |= ui.add(Slider::new(lacunarity, 1.0..=4.0).text("Lacunarity")).changed();
                    changed |= ui.add(Slider::new(gain, 0.0..=1.0).text("Gain (Persistence)")).changed();
                }
            }
            MaterialNode::MetalnessGenerator {
                threshold,
                falloff,
                detect_metals,
                invert,
            } => {
                changed |= ui.checkbox(detect_metals, "Smart Metal Detection (Gold, Copper, Silver)").changed();
                if *detect_metals {
                    changed |= ui.add(Slider::new(threshold, 0.1..=0.9).text("Silver Min Brightness")).changed();
                } else {
                    changed |= ui.add(Slider::new(threshold, 0.0..=1.0).text("Luminance Threshold")).changed();
                    changed |= ui.add(Slider::new(falloff, 0.01..=0.8).text("Edge Falloff")).changed();
                }
                changed |= ui.checkbox(invert, "Invert Output").changed();
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
                changed |= ui.add(Slider::new(min_lum, 0.0..=1.0).text("Min Luminance")).changed();
                changed |= ui.add(Slider::new(max_lum, 0.0..=1.0).text("Max Luminance")).changed();
                changed |= ui.checkbox(use_hue_filter, "Isolate Specific Color (Keying)").changed();
                if *use_hue_filter {
                    changed |= ui.add(Slider::new(target_hue, 0.0..=360.0).text("Target Hue (deg)")).changed();
                    changed |= ui.add(Slider::new(hue_tolerance, 5.0..=120.0).text("Hue Tolerance")).changed();
                }
                ui.horizontal(|ui| {
                    ui.label("Tint Color:");
                    changed |= ui.color_edit_button_rgb(tint_color).changed();
                });
                changed |= ui.add(Slider::new(intensity, 0.0..=20.0).text("Intensity Boost")).changed();
                changed |= ui.checkbox(invert, "Invert Luminance Mask").changed();
            }
            MaterialNode::CustomMapGenerator {
                model,
                param_a,
                param_b_color,
                invert_a,
                aniso_radial,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Shading Model:");
                    egui::ComboBox::from_id_salt((node_id, "cust_model"))
                        .selected_text(model.display_name())
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::Skin, "Skin (PreIntegrated SSS)").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::Anisotropy, "Anisotropy (Hair/Brushed)").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::Glass, "Glass").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::Cloth, "Cloth (Sheen/Fuzz)").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::ClearCoat, "Clear Coat").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::Subsurface, "Subsurface Profile").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::CelShading, "Cel Shading").changed();
                            changed |= ui.selectable_value(model, crate::graph::node::ShadingModel::ToonShading, "Toon Shading").changed();
                        });
                });

                ui.separator();
                ui.label(format!("Custom A ({})", model.custom_a_name()));
                changed |= ui.add(Slider::new(param_a, 0.0..=1.0).text("Param A")).changed();
                changed |= ui.checkbox(invert_a, "Invert A").changed();

                if *model != crate::graph::node::ShadingModel::ClearCoat {
                    ui.separator();
                    ui.label(format!("Custom B ({})", model.custom_b_name()));
                    if *model == crate::graph::node::ShadingModel::Anisotropy {
                        changed |= ui.checkbox(aniso_radial, "Radial Tangent Flow").changed();
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Color / Tint:");
                            changed |= ui.color_edit_button_rgb(param_b_color).changed();
                        });
                    }
                }
            }
            MaterialNode::RayMaterialOutput {
                material_name,
                shading_model,
                is_roughness_mode,
                emissive_intensity,
                emissive_blink_mode,
                emissive_blink_freq,
                hex_tiling_enable,
                hashed_alpha_enable,
                detail_map_enable,
                ..
            } => {
                ui.horizontal(|ui| {
                    ui.label("Material Name:");
                    changed |= ui.text_edit_singleline(material_name).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Shading Model:");
                    egui::ComboBox::from_id_salt((node_id, "out_model"))
                        .selected_text(shading_model.display_name())
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Default, "0: Default (Standard PBR)").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Skin, "1: Skin (PreIntegrated SSS)").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Anisotropy, "3: Anisotropy (Hair/Brushed)").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Glass, "4: Glass").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Cloth, "5: Cloth").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::ClearCoat, "6: Clear Coat").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Subsurface, "7: Subsurface Profile").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::CelShading, "8: Cel Shading").changed();
                            changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::ToonShading, "9: Toon Shading").changed();
                        });
                });
                changed |= ui.checkbox(is_roughness_mode, "Roughness Mode (Auto-invert for Ray-MMD Smoothness)").changed();

                ui.collapsing("Emissive Animation & Blink", |ui| {
                    changed |= ui.add(Slider::new(emissive_intensity, 0.0..=50.0).text("Intensity")).changed();
                    ui.horizontal(|ui| {
                        ui.label("Blink Mode:");
                        egui::ComboBox::from_id_salt((node_id, "blink_m"))
                            .selected_text(match *emissive_blink_mode {
                                0 => "None",
                                1 => "Constant Frequency",
                                _ => "Morph Controller",
                            })
                            .show_ui(ui, |ui| {
                                changed |= ui.selectable_value(emissive_blink_mode, 0, "None").changed();
                                changed |= ui.selectable_value(emissive_blink_mode, 1, "Constant Frequency").changed();
                                changed |= ui.selectable_value(emissive_blink_mode, 2, "Morph Controller").changed();
                            });
                    });
                    if *emissive_blink_mode == 1 {
                        ui.horizontal(|ui| {
                            ui.label("Freq (R,G,B):");
                            changed |= ui.add(DragValue::new(&mut emissive_blink_freq[0]).speed(0.1)).changed();
                            changed |= ui.add(DragValue::new(&mut emissive_blink_freq[1]).speed(0.1)).changed();
                            changed |= ui.add(DragValue::new(&mut emissive_blink_freq[2]).speed(0.1)).changed();
                        });
                    }
                });

                ui.separator();
                ui.label("ReForge Extended Features:");
                changed |= ui.checkbox(hex_tiling_enable, "Hex-Tiling (Mikkelsen 2022)").changed();
                changed |= ui.checkbox(hashed_alpha_enable, "Hashed Alpha Cutout").changed();
                changed |= ui.checkbox(detail_map_enable, "Detail Normal Micro-surface").changed();
            }
        }

        if changed {
            self.needs_rebuild = true;
        }
    }
}
