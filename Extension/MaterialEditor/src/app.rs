//! Main application state and UI for ReForge Material Editor.

use std::collections::HashMap;
use std::path::PathBuf;
use egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions, Vec2};
use egui_snarl::ui::SnarlWidget;
use egui_snarl::{InPinId, NodeId, OutPinId, Snarl};

use crate::graph::eval::{EvaluatedMaterial, GraphEvaluator};
use crate::graph::node::MaterialNode;
use crate::graph::viewer::MaterialSnarlViewer;
use crate::image_proc::*;
use crate::material_export::RayMaterialConfig;
use crate::viewport::{render_pbr_preview, PreviewCamera, PreviewPrimitive, ViewportDisplayMode};

/// Viewport display mode in the right panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportMode {
    Pbr3D,
    Map2D,
}

/// Color channel isolation filter for 2D map viewing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelFilter {
    Rgba,
    Red,
    Green,
    Blue,
    Alpha,
}

/// Tab selection for the bottom texture preview panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapViewTab {
    All,
    Albedo,
    Normal,
    Smoothness,
    Metalness,
    Occlusion,
    Parallax,
    Emissive,
    CustomA,
    CustomB,
}

impl MapViewTab {
    pub fn map_name(&self) -> Option<&'static str> {
        match self {
            MapViewTab::All => None,
            MapViewTab::Albedo => Some("Albedo"),
            MapViewTab::Normal => Some("Normal"),
            MapViewTab::Smoothness => Some("Smoothness"),
            MapViewTab::Metalness => Some("Metalness"),
            MapViewTab::Occlusion => Some("AO"),
            MapViewTab::Parallax => Some("Parallax"),
            MapViewTab::Emissive => Some("Emissive"),
            MapViewTab::CustomA => Some("Custom A"),
            MapViewTab::CustomB => Some("Custom B"),
        }
    }
}

/// Workspace mode: Single Material standalone vs PMX Model Studio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    SingleMaterial,
    PmxStudio,
}

/// Material slot for a PMX model subset.
pub struct PmxSubsetSlot {
    pub subset_index: usize,
    pub name: String,
    pub snarl: Snarl<MaterialNode>,
    pub evaluated: Option<EvaluatedMaterial>,
    pub fallback_texture: Option<U8Image>,
    pub fallback_handle: Option<TextureHandle>,
    pub is_dirty: bool,
    pub has_custom_material: bool,
    pub is_visible: bool,
}

/// Request message sent to the background evaluation thread.
pub struct EvalRequest {
    pub snarl: Snarl<MaterialNode>,
    pub working_resolution: u32,
    pub slot_index: Option<usize>,
}

/// Response message received from the background evaluation thread.
pub struct EvalResponse {
    pub evaluated: EvaluatedMaterial,
    pub eval_time_ms: f32,
    pub slot_index: Option<usize>,
}

pub struct MaterialEditorApp {
    pub app_mode: AppMode,

    snarl: Snarl<MaterialNode>,
    viewer: MaterialSnarlViewer,
    evaluated: EvaluatedMaterial,
    camera: PreviewCamera,
    preview_texture: Option<TextureHandle>,
    channel_textures: HashMap<String, TextureHandle>,
    active_tab: MapViewTab,
    status_message: String,
    last_eval_time_ms: f32,
    preview_resolution: u32,
    working_resolution: u32,
    export_target_dir: Option<PathBuf>,
    graph_dirty: bool,
    preview_dirty: bool,
    is_interacting: bool,
    is_evaluating: bool,
    pending_eval: bool,
    eval_tx: std::sync::mpsc::Sender<EvalRequest>,
    eval_rx: std::sync::mpsc::Receiver<EvalResponse>,

    // 2D Texture Map Viewer State
    viewport_mode: ViewportMode,
    selected_map_name: String,
    channel_filter: ChannelFilter,
    map_zoom: f32,
    map_pan: egui::Vec2,
    full_map_texture: Option<(String, TextureHandle)>,
    full_map_dirty: bool,
    show_map_inspector_window: bool,

    // PMX Model Studio State
    pub pmx_model: Option<crate::pmx::PmxModel>,
    pub pmx_slots: Vec<PmxSubsetSlot>,
    pub active_pmx_subset: Option<usize>,
    pub solo_active_subset: bool,
    pub pmx_camera: PreviewCamera,
    pub pmx_preview_texture: Option<TextureHandle>,
    pub pmx_preview_dirty: bool,
    pub pmx_search_filter: String,
    pub pmx_center_view_mode: usize, // 0: Graph, 1: Full 3D
    pub pmx_viewport_size: egui::Vec2,
}

impl MaterialEditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (eval_tx, req_rx) = std::sync::mpsc::channel::<EvalRequest>();
        let (res_tx, eval_rx) = std::sync::mpsc::channel::<EvalResponse>();

        std::thread::Builder::new()
            .name("material-eval-worker".to_string())
            .spawn(move || {
                while let Ok(req) = req_rx.recv() {
                    let mut latest_req = req;
                    while let Ok(newer) = req_rx.try_recv() {
                        if newer.slot_index == latest_req.slot_index {
                            latest_req = newer;
                        } else {
                            let start = std::time::Instant::now();
                            let mut evaluator = GraphEvaluator::with_resolution(&latest_req.snarl, latest_req.working_resolution);
                            let evaluated = evaluator.evaluate_material();
                            let eval_time_ms = start.elapsed().as_secs_f32() * 1000.0;

                            let _ = res_tx.send(EvalResponse {
                                evaluated,
                                eval_time_ms,
                                slot_index: latest_req.slot_index,
                            });
                            latest_req = newer;
                        }
                    }

                    let start = std::time::Instant::now();
                    let mut evaluator = GraphEvaluator::with_resolution(&latest_req.snarl, latest_req.working_resolution);
                    let evaluated = evaluator.evaluate_material();
                    let eval_time_ms = start.elapsed().as_secs_f32() * 1000.0;

                    let _ = res_tx.send(EvalResponse {
                        evaluated,
                        eval_time_ms,
                        slot_index: latest_req.slot_index,
                    });
                }
            })
            .expect("Failed to spawn material eval worker");

        let mut app = Self {
            app_mode: AppMode::SingleMaterial,
            snarl: Snarl::new(),
            viewer: MaterialSnarlViewer::new(),
            evaluated: EvaluatedMaterial::default(),
            camera: PreviewCamera::default(),
            preview_texture: None,
            channel_textures: HashMap::new(),
            active_tab: MapViewTab::All,
            status_message: "Ready. Add nodes or load a preset to begin.".to_string(),
            last_eval_time_ms: 0.0,
            preview_resolution: 384,
            working_resolution: 512,
            export_target_dir: None,
            graph_dirty: true,
            preview_dirty: true,
            is_interacting: false,
            is_evaluating: false,
            pending_eval: false,
            eval_tx,
            eval_rx,

            viewport_mode: ViewportMode::Pbr3D,
            selected_map_name: "Albedo".to_string(),
            channel_filter: ChannelFilter::Rgba,
            map_zoom: 1.0,
            map_pan: egui::Vec2::ZERO,
            full_map_texture: None,
            full_map_dirty: true,
            show_map_inspector_window: false,

            pmx_model: None,
            pmx_slots: Vec::new(),
            active_pmx_subset: None,
            solo_active_subset: false,
            pmx_camera: PreviewCamera::default(),
            pmx_preview_texture: None,
            pmx_preview_dirty: false,
            pmx_search_filter: String::new(),
            pmx_center_view_mode: 0,
            pmx_viewport_size: egui::vec2(512.0, 512.0),
        };

        app.load_default_preset();
        let init_req = EvalRequest {
            snarl: app.snarl.clone(),
            working_resolution: app.working_resolution,
            slot_index: None,
        };
        let _ = app.eval_tx.send(init_req);
        app.is_evaluating = true;
        app
    }

    /// Sets up default PBR graph with Diffuse -> Height -> Normal -> AO -> Material Output.
    pub fn load_default_preset(&mut self) {
        self.snarl = Snarl::new();

        // 1. Color Input (Base Color)
        let color_node = self.snarl.insert_node(
            egui::pos2(80.0, 100.0),
            MaterialNode::ColorInput {
                color: [0.75, 0.72, 0.68, 1.0],
            },
        );

        // 2. Height Generator
        let height_node = self.snarl.insert_node(
            egui::pos2(320.0, 100.0),
            MaterialNode::HeightGenerator {
                contrast: 1.5,
                brightness: 0.0,
                invert: false,
            },
        );

        // 3. Normal Generator (Sobel DirectX)
        let normal_node = self.snarl.insert_node(
            egui::pos2(560.0, 60.0),
            MaterialNode::NormalGenerator {
                scale: 2.0,
                filter: NormalFilter::Scharr,
                orientation: NormalOrientation::DirectX,
            },
        );

        // 4. AO Generator
        let ao_node = self.snarl.insert_node(
            egui::pos2(560.0, 240.0),
            MaterialNode::AOGenerator {
                radius: 16,
                samples: 16,
                intensity: 1.5,
                bias: 0.05,
            },
        );

        // 5. Roughness Generator
        let rough_node = self.snarl.insert_node(
            egui::pos2(560.0, 420.0),
            MaterialNode::RoughnessGenerator {
                invert: false,
                contrast: 1.2,
                min_val: 0.2,
                max_val: 0.7,
            },
        );

        // 6. Master Material Output
        let output_node = self.snarl.insert_node(
            egui::pos2(860.0, 100.0),
            MaterialNode::RayMaterialOutput {
                material_name: "reforge_material".to_string(),
                shading_model: crate::graph::node::ShadingModel::Default,
                albedo_color: [1.0, 1.0, 1.0],
                albedo_loop: [1.0, 1.0],
                normal_scale: 1.0,
                normal_loop: 1.0,
                smoothness_val: 0.5,
                is_roughness_mode: true,
                metalness_val: 0.0,
                specular_color: [0.5, 0.5, 0.5],
                occlusion_val: 1.0,
                parallax_scale: 0.05,
                emissive_color: [1.0, 1.0, 1.0],
                emissive_intensity: 1.0,
                emissive_blink_mode: 0,
                emissive_blink_freq: [1.0, 1.0, 1.0],
                custom_a_val: 0.0,
                custom_b_color: [0.0, 0.0, 0.0],
                hex_tiling_enable: false,
                hex_tiling_rotation: 1.0,
                hex_tiling_contrast: 0.6,
                hex_tiling_sharpness: 7.0,
                hashed_alpha_enable: false,
                hashed_alpha_scale: 1.0,
                detail_map_enable: false,
                detail_normal_scale: 1.0,
                detail_normal_loop: 20.0,
                detail_fade_distance: 15.0,
            },
        );

        // Connect graph wires
        self.connect(color_node, 0, height_node, 0);
        self.connect(height_node, 0, normal_node, 0);
        self.connect(height_node, 0, ao_node, 0);
        self.connect(height_node, 0, rough_node, 0);

        // Connect to Master Output:
        // 0: Albedo, 3: Normal, 4: Smoothness, 7: AO, 8: Height
        self.connect(color_node, 0, output_node, 0);
        self.connect(normal_node, 0, output_node, 3);
        self.connect(rough_node, 0, output_node, 4);
        self.connect(ao_node, 0, output_node, 7);
        self.connect(height_node, 0, output_node, 8);

        self.graph_dirty = true;
    }

    /// Connects output pin to input pin.
    fn connect(&mut self, from_node: NodeId, out_idx: usize, to_node: NodeId, in_idx: usize) {
        self.snarl.connect(
            OutPinId {
                node: from_node,
                output: out_idx,
            },
            InPinId {
                node: to_node,
                input: in_idx,
            },
        );
    }

    /// Updates channel preview textures for gallery using lightweight thumbnails to eliminate GPU memory lag.
    pub fn update_channel_textures(&mut self, ctx: &Context) {
        let mut thumbs: Vec<(&'static str, Option<ColorImage>)> = Vec::with_capacity(9);
        {
            let eval = if self.app_mode == AppMode::PmxStudio {
                self.active_pmx_subset
                    .and_then(|i| self.pmx_slots.get(i))
                    .and_then(|s| s.evaluated.as_ref())
                    .unwrap_or(&self.evaluated)
            } else {
                &self.evaluated
            };

            let channels = [
                ("Albedo", eval.albedo.as_ref()),
                ("Normal", eval.normal.as_ref()),
                ("Smoothness", eval.smoothness.as_ref()),
                ("Metalness", eval.metalness.as_ref()),
                ("AO", eval.occlusion.as_ref()),
                ("Parallax", eval.parallax.as_ref()),
                ("Emissive", eval.emissive.as_ref()),
                ("Custom A", eval.custom_a.as_ref()),
                ("Custom B", eval.custom_b.as_ref()),
            ];

            for (name, img_opt) in channels {
                if let Some(m) = img_opt {
                    let thumb = if m.width() > 128 || m.height() > 128 {
                        image::imageops::resize(m, 128, 128, image::imageops::FilterType::Triangle)
                    } else {
                        m.clone()
                    };
                    thumbs.push((
                        name,
                        Some(ColorImage::from_rgba_unmultiplied(
                            [thumb.width() as usize, thumb.height() as usize],
                            thumb.as_flat_samples().as_slice(),
                        )),
                    ));
                } else {
                    thumbs.push((name, None));
                }
            }
        }

        for (name, ci_opt) in thumbs {
            if let Some(ci) = ci_opt {
                if let Some(tex) = self.channel_textures.get_mut(name) {
                    tex.set(ci, TextureOptions::LINEAR);
                } else {
                    self.channel_textures.insert(name.to_string(), ctx.load_texture(name, ci, TextureOptions::LINEAR));
                }
            } else {
                self.channel_textures.remove(name);
            }
        }
    }

    /// Helper to get a reference to the evaluated map by name.
    pub fn get_evaluated_map(&self, name: &str) -> Option<&U8Image> {
        let eval = if self.app_mode == AppMode::PmxStudio {
            self.active_pmx_subset
                .and_then(|i| self.pmx_slots.get(i))
                .and_then(|s| s.evaluated.as_ref())
                .unwrap_or(&self.evaluated)
        } else {
            &self.evaluated
        };

        match name {
            "Albedo" => eval.albedo.as_ref(),
            "Normal" => eval.normal.as_ref(),
            "Smoothness" | "Roughness" => eval.smoothness.as_ref(),
            "Metalness" => eval.metalness.as_ref(),
            "AO" | "Occlusion" => eval.occlusion.as_ref(),
            "Parallax" | "Height" => eval.parallax.as_ref(),
            "Emissive" => eval.emissive.as_ref(),
            "Custom A" => eval.custom_a.as_ref(),
            "Custom B" => eval.custom_b.as_ref(),
            "Detail Normal" => eval.detail_normal.as_ref(),
            _ => None,
        }
    }

    /// Updates the full-resolution texture used for 2D inspection with optional channel filtering.
    pub fn update_full_map_texture(&mut self, ctx: &Context) {
        let img = self.get_evaluated_map(&self.selected_map_name);
        if let Some(src) = img {
            let filtered_ci = match self.channel_filter {
                ChannelFilter::Rgba => ColorImage::from_rgba_unmultiplied(
                    [src.width() as usize, src.height() as usize],
                    src.as_flat_samples().as_slice(),
                ),
                ChannelFilter::Red => {
                    let mut pixels = Vec::with_capacity((src.width() * src.height() * 4) as usize);
                    for p in src.pixels() {
                        pixels.extend_from_slice(&[p[0], p[0], p[0], 255]);
                    }
                    ColorImage::from_rgba_unmultiplied([src.width() as usize, src.height() as usize], &pixels)
                }
                ChannelFilter::Green => {
                    let mut pixels = Vec::with_capacity((src.width() * src.height() * 4) as usize);
                    for p in src.pixels() {
                        pixels.extend_from_slice(&[p[1], p[1], p[1], 255]);
                    }
                    ColorImage::from_rgba_unmultiplied([src.width() as usize, src.height() as usize], &pixels)
                }
                ChannelFilter::Blue => {
                    let mut pixels = Vec::with_capacity((src.width() * src.height() * 4) as usize);
                    for p in src.pixels() {
                        pixels.extend_from_slice(&[p[2], p[2], p[2], 255]);
                    }
                    ColorImage::from_rgba_unmultiplied([src.width() as usize, src.height() as usize], &pixels)
                }
                ChannelFilter::Alpha => {
                    let mut pixels = Vec::with_capacity((src.width() * src.height() * 4) as usize);
                    for p in src.pixels() {
                        pixels.extend_from_slice(&[p[3], p[3], p[3], 255]);
                    }
                    ColorImage::from_rgba_unmultiplied([src.width() as usize, src.height() as usize], &pixels)
                }
            };

            if let Some((ref name, ref mut tex)) = self.full_map_texture {
                if name == &self.selected_map_name {
                    tex.set(filtered_ci, TextureOptions::LINEAR);
                    return;
                }
            }
            let handle = ctx.load_texture("inspected_map", filtered_ci, TextureOptions::LINEAR);
            self.full_map_texture = Some((self.selected_map_name.clone(), handle));
        } else {
            self.full_map_texture = None;
        }
    }

    /// Saves a single texture map to disk in PNG/TGA/BMP format.
    pub fn save_single_map(&self, name: &str) {
        let img = match self.get_evaluated_map(name) {
            Some(m) => m,
            None => return,
        };
        let default_name = format!("{}.png", name.to_lowercase().replace(' ', "_"));
        if let Some(path) = rfd::FileDialog::new()
            .set_title(&format!("Save {} Map", name))
            .set_file_name(&default_name)
            .add_filter("PNG Image", &["png"])
            .add_filter("TGA Image", &["tga"])
            .add_filter("BMP Image", &["bmp"])
            .save_file()
        {
            let _ = img.save(path);
        }
    }

    /// Renders the interactive 2D texture map viewer.
    pub fn show_2d_map_viewer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Map:");
            let old_map = self.selected_map_name.clone();
            egui::ComboBox::from_id_salt("map_selector_2d")
                .selected_text(&self.selected_map_name)
                .show_ui(ui, |ui| {
                    for name in &["Albedo", "Normal", "Smoothness", "Metalness", "AO", "Parallax", "Emissive", "Custom A", "Custom B"] {
                        ui.selectable_value(&mut self.selected_map_name, name.to_string(), *name);
                    }
                });
            if old_map != self.selected_map_name {
                self.full_map_dirty = true;
            }

            ui.separator();
            ui.label("Channels:");
            let old_filter = self.channel_filter;
            ui.selectable_value(&mut self.channel_filter, ChannelFilter::Rgba, "RGB");
            ui.selectable_value(&mut self.channel_filter, ChannelFilter::Red, "R");
            ui.selectable_value(&mut self.channel_filter, ChannelFilter::Green, "G");
            ui.selectable_value(&mut self.channel_filter, ChannelFilter::Blue, "B");
            ui.selectable_value(&mut self.channel_filter, ChannelFilter::Alpha, "A");
            if old_filter != self.channel_filter {
                self.full_map_dirty = true;
            }

            ui.separator();
            if ui.button("💾 Save Image...").clicked() {
                self.save_single_map(&self.selected_map_name);
            }
        });

        if let Some(map_img) = self.get_evaluated_map(&self.selected_map_name) {
            ui.small(format!("Dimensions: {} × {} px", map_img.width(), map_img.height()));
        } else {
            ui.colored_label(Color32::GRAY, "No texture data generated for this map channel.");
        }

        ui.separator();

        // 2D Pan and Zoom Canvas
        let avail_size = ui.available_size();
        let (canvas_rect, canvas_resp) = ui.allocate_exact_size(
            Vec2::new(avail_size.x.max(100.0), avail_size.y.max(100.0)),
            egui::Sense::drag(),
        );

        // Handle panning
        if canvas_resp.dragged() {
            self.map_pan += canvas_resp.drag_delta();
        }

        // Handle zooming with mouse wheel
        if canvas_resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > 0.1 {
                let zoom_factor = if scroll > 0.0 { 1.15 } else { 0.85 };
                self.map_zoom = (self.map_zoom * zoom_factor).clamp(0.05, 20.0);
            }
        }

        // Draw canvas background
        let painter = ui.painter_at(canvas_rect);
        painter.rect_filled(canvas_rect, 0.0, Color32::from_rgb(18, 18, 22));

        if let Some((_, ref tex)) = self.full_map_texture {
            let tex_size = tex.size_vec2();
            let aspect = tex_size.x / tex_size.y.max(1.0);
            let base_fit_w = canvas_rect.width().min(canvas_rect.height() * aspect) * 0.9;
            let base_fit_h = base_fit_w / aspect;

            let draw_w = base_fit_w * self.map_zoom;
            let draw_h = base_fit_h * self.map_zoom;
            let center = canvas_rect.center() + self.map_pan;
            let img_rect = egui::Rect::from_center_size(center, Vec2::new(draw_w, draw_h));

            // Draw checkerboard background for transparent areas
            painter.rect_filled(img_rect, 0.0, Color32::from_rgb(32, 32, 36));

            // Draw texture map
            painter.image(
                tex.id(),
                img_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );

            // Draw border around map
            painter.rect_stroke(img_rect, 0.0, egui::Stroke::new(1.0, Color32::from_rgb(80, 80, 100)), egui::StrokeKind::Outside);

            // Reset Pan / Zoom Button in corner
            let reset_rect = egui::Rect::from_min_size(canvas_rect.min + Vec2::new(8.0, 8.0), Vec2::new(90.0, 24.0));
            if ui.put(reset_rect, egui::Button::new("Reset View")).clicked() {
                self.map_pan = Vec2::ZERO;
                self.map_zoom = 1.0;
            }

            // Pixel probe under cursor
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if img_rect.contains(mouse_pos) {
                    if let Some(map_img) = self.get_evaluated_map(&self.selected_map_name) {
                        let u = ((mouse_pos.x - img_rect.min.x) / img_rect.width()).clamp(0.0, 1.0);
                        let v = ((mouse_pos.y - img_rect.min.y) / img_rect.height()).clamp(0.0, 1.0);
                        let px = (u * (map_img.width() - 1) as f32).round() as u32;
                        let py = (v * (map_img.height() - 1) as f32).round() as u32;
                        let pixel = map_img.get_pixel(px, py);

                        let info_text = format!(
                            "Pixel [{}, {}]  RGB: [{}, {}, {}]  Alpha: {}  Hex: #{:02X}{:02X}{:02X}",
                            px, py, pixel[0], pixel[1], pixel[2], pixel[3], pixel[0], pixel[1], pixel[2]
                        );

                        let probe_rect = egui::Rect::from_min_size(
                            canvas_rect.min + Vec2::new(8.0, canvas_rect.height() - 32.0),
                            Vec2::new(320.0, 24.0),
                        );
                        painter.rect_filled(probe_rect, 4.0, Color32::from_rgba_premultiplied(0, 0, 0, 200));
                        painter.text(
                            probe_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            info_text,
                            egui::FontId::monospace(11.0),
                            Color32::from_rgb(220, 220, 230),
                        );
                    }
                }
            }
        }
    }

    /// Renders the 3D PBR raymarching preview (fast-path during interaction, high-res when idle).
    pub fn render_preview(&mut self, ctx: &Context) {
        let res = if self.is_interacting {
            192
        } else {
            self.preview_resolution
        };

        let preview_img = render_pbr_preview(
            &self.evaluated,
            &self.camera,
            res,
            res,
        );

        let color_image = ColorImage::from_rgba_unmultiplied(
            [preview_img.width() as usize, preview_img.height() as usize],
            preview_img.as_flat_samples().as_slice(),
        );
        self.preview_texture = Some(ctx.load_texture(
            "pbr_preview",
            color_image,
            TextureOptions::LINEAR,
        ));
    }

    /// Exports the full Ray-MMD material package (.fx + baked textures at full quality).
    pub fn export_ray_material(&mut self) {
        let target_dir = match rfd::FileDialog::new()
            .set_title("Select Export Directory for Ray-MMD Material")
            .pick_folder()
        {
            Some(dir) => dir,
            None => return,
        };

        // For Ray-MMD export, synchronously evaluate at 100% full source resolution (0 = Full Source)
        let mut evaluator = GraphEvaluator::with_resolution(&self.snarl, 0);
        let baked = evaluator.evaluate_material();

        let mut config = RayMaterialConfig::default();

        // Extract parameters from RayMaterialOutput node
        for node in self.snarl.nodes() {
            if let MaterialNode::RayMaterialOutput {
                material_name,
                shading_model,
                albedo_color,
                albedo_loop,
                normal_scale,
                normal_loop,
                smoothness_val,
                is_roughness_mode,
                metalness_val,
                specular_color,
                occlusion_val,
                parallax_scale,
                emissive_color,
                emissive_intensity,
                emissive_blink_mode,
                emissive_blink_freq,
                custom_a_val,
                custom_b_color,
                hex_tiling_enable,
                hex_tiling_rotation,
                hex_tiling_contrast,
                hex_tiling_sharpness,
                hashed_alpha_enable,
                hashed_alpha_scale,
                detail_map_enable,
                detail_normal_scale,
                detail_normal_loop,
                detail_fade_distance,
            } = node
            {
                config.name = material_name.clone();
                config.shading_model_id = shading_model.id();
                config.custom_enabled = shading_model.id() > 0;
                config.albedo_color = *albedo_color;
                config.albedo_loop = *albedo_loop;
                config.normal_scale = *normal_scale;
                config.normal_loop = *normal_loop;
                config.smoothness_val = *smoothness_val;
                config.is_roughness_mode = *is_roughness_mode;
                config.metalness_val = *metalness_val;
                config.specular_color = *specular_color;
                config.occlusion_val = *occlusion_val;
                config.parallax_scale = *parallax_scale;
                config.emissive_color = *emissive_color;
                config.emissive_intensity = *emissive_intensity;
                config.emissive_blink_mode = *emissive_blink_mode;
                config.emissive_blink = *emissive_blink_freq;
                config.custom_a_val = *custom_a_val;
                config.custom_b_color = *custom_b_color;

                config.hex_tiling_enable = *hex_tiling_enable;
                config.hex_tiling_rotation = *hex_tiling_rotation;
                config.hex_tiling_contrast = *hex_tiling_contrast;
                config.hex_tiling_sharpness = *hex_tiling_sharpness;

                config.hashed_alpha_enable = *hashed_alpha_enable;
                config.hashed_alpha_scale = *hashed_alpha_scale;

                config.detail_map_enable = *detail_map_enable;
                config.detail_normal_scale = *detail_normal_scale;
                config.detail_normal_loop = *detail_normal_loop;
                config.detail_fade_distance = *detail_fade_distance;
            }
        }

        // Collect textures to save from baked full-res maps
        let mut textures_to_export = Vec::new();
        if let Some(ref alb) = baked.albedo {
            config.albedo_enabled = true;
            textures_to_export.push(("albedo.png".to_string(), alb));
        }
        if let Some(ref nrm) = baked.normal {
            config.normal_enabled = true;
            textures_to_export.push(("normal.png".to_string(), nrm));
        }
        if let Some(ref smt) = baked.smoothness {
            config.smoothness_enabled = true;
            textures_to_export.push(("smoothness.png".to_string(), smt));
        }
        if let Some(ref mtl) = baked.metalness {
            config.metalness_enabled = true;
            textures_to_export.push(("metalness.png".to_string(), mtl));
        }
        if let Some(ref occ) = baked.occlusion {
            config.occlusion_enabled = true;
            textures_to_export.push(("occlusion.png".to_string(), occ));
        }
        if let Some(ref par) = baked.parallax {
            config.parallax_enabled = true;
            textures_to_export.push(("height.png".to_string(), par));
        }
        if let Some(ref emi) = baked.emissive {
            config.emissive_enabled = true;
            textures_to_export.push(("emissive.png".to_string(), emi));
        }
        if let Some(ref cust_a) = baked.custom_a {
            config.custom_enabled = true;
            config.custom_a_enabled = true;
            textures_to_export.push(("custom_a.png".to_string(), cust_a));
        }
        if let Some(ref cust_b) = baked.custom_b {
            config.custom_enabled = true;
            config.custom_b_enabled = true;
            textures_to_export.push(("custom_b.png".to_string(), cust_b));
        }
        if let Some(ref det) = baked.detail_normal {
            config.detail_map_enable = true;
            textures_to_export.push(("detail_normal.png".to_string(), det));
        }

        match config.export_package(&target_dir, &textures_to_export) {
            Ok(_) => {
                self.status_message = format!("Exported successfully to {:?}", target_dir);
                self.export_target_dir = Some(target_dir);
            }
            Err(e) => {
                self.status_message = format!("Export failed: {:#}", e);
            }
        }
    }

    /// Loads a PMX model and sets up multi-material subset project.
    pub fn load_pmx_file<P: AsRef<std::path::Path>>(&mut self, path: P, ctx: &Context) {
        let path_ref = path.as_ref();
        match crate::pmx::PmxModel::load_from_file(path_ref) {
            Ok(model) => {
                let mut slots = Vec::with_capacity(model.subsets.len());

                for (idx, subset) in model.subsets.iter().enumerate() {
                    let mut snarl = Snarl::new();
                    let name = if !subset.name_universal.is_empty() {
                        subset.name_universal.clone()
                    } else if !subset.name_local.is_empty() {
                        subset.name_local.clone()
                    } else {
                        format!("Subset_{:02}", idx)
                    };

                    let out_node = snarl.insert_node(
                        egui::pos2(600.0, 100.0),
                        MaterialNode::RayMaterialOutput {
                            material_name: name.replace(' ', "_"),
                            shading_model: crate::graph::node::ShadingModel::Default,
                            albedo_color: [subset.diffuse[0], subset.diffuse[1], subset.diffuse[2]],
                            albedo_loop: [1.0, 1.0],
                            normal_scale: 1.0,
                            normal_loop: 1.0,
                            smoothness_val: 0.5,
                            is_roughness_mode: false,
                            metalness_val: 0.0,
                            specular_color: subset.specular,
                            occlusion_val: 1.0,
                            parallax_scale: 0.05,
                            emissive_color: [1.0, 1.0, 1.0],
                            emissive_intensity: 1.0,
                            emissive_blink_mode: 0,
                            emissive_blink_freq: [1.0, 1.0, 1.0],
                            custom_a_val: 0.0,
                            custom_b_color: [0.0, 0.0, 0.0],
                            hex_tiling_enable: false,
                            hex_tiling_rotation: 1.0,
                            hex_tiling_contrast: 0.6,
                            hex_tiling_sharpness: 7.0,
                            hashed_alpha_enable: false,
                            hashed_alpha_scale: 1.0,
                            detail_map_enable: false,
                            detail_normal_scale: 1.0,
                            detail_normal_loop: 20.0,
                            detail_fade_distance: 15.0,
                        },
                    );

                    let mut fallback_tex = None;
                    let mut fallback_handle = None;
                    if let Some(ref tex_path) = subset.absolute_texture_path {
                        if tex_path.exists() {
                            if let Ok(dyn_img) = image::open(tex_path) {
                                let rgba = dyn_img.to_rgba8();
                                let tex_handle = ctx.load_texture(
                                    format!("pmx_sub_{}", idx),
                                    ColorImage::from_rgba_unmultiplied(
                                        [rgba.width() as usize, rgba.height() as usize],
                                        rgba.as_flat_samples().as_slice(),
                                    ),
                                    TextureOptions::LINEAR,
                                );
                                fallback_handle = Some(tex_handle);

                                let img_node = snarl.insert_node(
                                    egui::pos2(150.0, 100.0),
                                    MaterialNode::ImageInput {
                                        file_path: tex_path.to_string_lossy().to_string(),
                                        is_srgb: true,
                                        cached_image: Some(rgba.clone()),
                                    },
                                );
                                snarl.connect(
                                    OutPinId { node: img_node, output: 0 },
                                    InPinId { node: out_node, input: 0 },
                                );
                                fallback_tex = Some(rgba);
                            }
                        }
                    }

                    slots.push(PmxSubsetSlot {
                        subset_index: idx,
                        name,
                        snarl,
                        evaluated: None,
                        fallback_texture: fallback_tex,
                        fallback_handle,
                        is_dirty: true,
                        has_custom_material: false,
                        is_visible: true,
                    });
                }

                let is_stage = model.radius > 50.0 || (model.bbox_max.x - model.bbox_min.x) > 80.0;
                if is_stage {
                    self.pmx_camera.target = glam::Vec3::new(0.0, 8.0, 10.0);
                    self.pmx_camera.distance = 65.0;
                    self.pmx_camera.pitch = 0.18;
                    self.pmx_camera.yaw = 0.0;
                } else {
                    self.pmx_camera.target = model.center;
                    self.pmx_camera.distance = (model.radius * 2.5).max(10.0);
                    self.pmx_camera.pitch = 0.2;
                    self.pmx_camera.yaw = 0.0;
                }
                self.pmx_model = Some(model);
                self.pmx_slots = slots;
                self.active_pmx_subset = if !self.pmx_slots.is_empty() { Some(0) } else { None };
                self.app_mode = AppMode::PmxStudio;
                self.pmx_preview_dirty = true;
                self.status_message = format!("Loaded PMX model with {} subsets.", self.pmx_slots.len());
            }
            Err(e) => {
                self.status_message = format!("Failed to load PMX model: {}", e);
            }
        }
    }

    /// Renders the PMX 3D model and uploads the texture to GPU.
    pub fn update_pmx_preview(&mut self, ctx: &Context) {
        let model = match self.pmx_model.as_ref() {
            Some(m) => m,
            None => return,
        };

        let base_res = if self.is_interacting { 280.0 } else { self.preview_resolution as f32 };
        let aspect = if self.pmx_viewport_size.x > 10.0 && self.pmx_viewport_size.y > 10.0 {
            (self.pmx_viewport_size.x / self.pmx_viewport_size.y).clamp(0.2, 5.0)
        } else {
            1.0
        };

        let (width, height) = if aspect >= 1.0 {
            let w = (base_res * aspect).round() as u32;
            let h = base_res as u32;
            (w.clamp(64, 1920), h.clamp(64, 1920))
        } else {
            let w = base_res as u32;
            let h = (base_res / aspect).round() as u32;
            (w.clamp(64, 1920), h.clamp(64, 1920))
        };

        let subset_materials: Vec<Option<EvaluatedMaterial>> = self
            .pmx_slots
            .iter()
            .map(|s| s.evaluated.clone())
            .collect();
        let fallback_textures: Vec<Option<U8Image>> = self
            .pmx_slots
            .iter()
            .map(|s| s.fallback_texture.clone())
            .collect();

        let shading_model = if let Some(idx) = self.active_pmx_subset {
            self.pmx_slots.get(idx)
                .and_then(|slot| {
                    for node in slot.snarl.nodes() {
                        if let MaterialNode::RayMaterialOutput { shading_model, .. } = node {
                            return Some(*shading_model);
                        }
                    }
                    None
                })
                .unwrap_or(crate::graph::node::ShadingModel::Default)
        } else {
            crate::graph::node::ShadingModel::Default
        };

        let img = crate::pmx::render_pmx_model(
            model,
            &subset_materials,
            &fallback_textures,
            &self.pmx_camera,
            shading_model,
            self.pmx_camera.display_mode,
            self.active_pmx_subset,
            self.solo_active_subset,
            width,
            height,
        );

        let ci = ColorImage::from_rgba_unmultiplied(
            [img.width() as usize, img.height() as usize],
            img.as_flat_samples().as_slice(),
        );

        if let Some(ref mut handle) = self.pmx_preview_texture {
            handle.set(ci, TextureOptions::LINEAR);
        } else {
            self.pmx_preview_texture = Some(ctx.load_texture("pmx_preview_tex", ci, TextureOptions::LINEAR));
        }

        self.pmx_preview_dirty = false;
    }

    /// Exports all PMX subsets as ready-to-use Ray-MMD materials (.fx + maps).
    pub fn export_all_pmx_materials(&mut self) {
        let model = match self.pmx_model.as_ref() {
            Some(m) => m,
            None => {
                self.status_message = "No PMX model loaded to export.".to_string();
                return;
            }
        };

        let base_dir = match model.file_path.as_ref().and_then(|p| p.parent()) {
            Some(p) => p.to_path_buf(),
            None => {
                if let Some(picked) = rfd::FileDialog::new().pick_folder() {
                    picked
                } else {
                    return;
                }
            }
        };

        let export_dir = base_dir.join("materials_reforge");
        let _ = std::fs::create_dir_all(&export_dir);

        let mut exported_count = 0;

        for (idx, slot) in self.pmx_slots.iter().enumerate() {
            let clean_name = format!("{:02}_{}", idx, slot.name.replace(' ', "_").replace(|c: char| !c.is_alphanumeric() && c != '_', ""));

            let evaluated = if let Some(ref ev) = slot.evaluated {
                ev.clone()
            } else {
                let mut eval = GraphEvaluator::with_resolution(&slot.snarl, 1024);
                eval.evaluate_material()
            };

            let mut config = RayMaterialConfig::default();
            config.name = clean_name.clone();

            for node in slot.snarl.nodes() {
                if let MaterialNode::RayMaterialOutput {
                    shading_model,
                    albedo_color,
                    albedo_loop,
                    normal_scale,
                    normal_loop,
                    smoothness_val,
                    is_roughness_mode,
                    metalness_val,
                    specular_color,
                    occlusion_val,
                    parallax_scale,
                    emissive_color,
                    emissive_intensity,
                    emissive_blink_mode,
                    emissive_blink_freq,
                    custom_a_val,
                    custom_b_color,
                    hex_tiling_enable,
                    hex_tiling_rotation,
                    hex_tiling_contrast,
                    hex_tiling_sharpness,
                    hashed_alpha_enable,
                    hashed_alpha_scale,
                    detail_map_enable,
                    detail_normal_scale,
                    detail_normal_loop,
                    detail_fade_distance,
                    ..
                } = node {
                    config.shading_model_id = shading_model.id();
                    config.custom_enabled = shading_model.id() > 0;
                    config.albedo_color = *albedo_color;
                    config.albedo_loop = *albedo_loop;
                    config.normal_scale = *normal_scale;
                    config.normal_loop = *normal_loop;
                    config.smoothness_val = *smoothness_val;
                    config.is_roughness_mode = *is_roughness_mode;
                    config.metalness_val = *metalness_val;
                    config.specular_color = *specular_color;
                    config.occlusion_val = *occlusion_val;
                    config.parallax_scale = *parallax_scale;
                    config.emissive_color = *emissive_color;
                    config.emissive_intensity = *emissive_intensity;
                    config.emissive_blink_mode = *emissive_blink_mode;
                    config.emissive_blink = *emissive_blink_freq;
                    config.custom_a_val = *custom_a_val;
                    config.custom_b_color = *custom_b_color;
                    config.hex_tiling_enable = *hex_tiling_enable;
                    config.hex_tiling_rotation = *hex_tiling_rotation;
                    config.hex_tiling_contrast = *hex_tiling_contrast;
                    config.hex_tiling_sharpness = *hex_tiling_sharpness;
                    config.hashed_alpha_enable = *hashed_alpha_enable;
                    config.hashed_alpha_scale = *hashed_alpha_scale;
                    config.detail_map_enable = *detail_map_enable;
                    config.detail_normal_scale = *detail_normal_scale;
                    config.detail_normal_loop = *detail_normal_loop;
                    config.detail_fade_distance = *detail_fade_distance;
                }
            }

            if let Some(ref alb) = evaluated.albedo {
                let fname = format!("{}_albedo.png", clean_name);
                let _ = alb.save(export_dir.join(&fname));
                config.albedo_file = fname;
                config.albedo_enabled = true;
            }
            if let Some(ref n) = evaluated.normal {
                let fname = format!("{}_normal.png", clean_name);
                let mut packed_n = n.clone();
                if let Some(ref s) = evaluated.smoothness {
                    for (x, y, pix) in packed_n.enumerate_pixels_mut() {
                        let (sw, sh) = s.dimensions();
                        let sx = (x * sw / n.width()).min(sw - 1);
                        let sy = (y * sh / n.height()).min(sh - 1);
                        pix[3] = s.get_pixel(sx, sy)[0];
                    }
                }
                let _ = packed_n.save(export_dir.join(&fname));
                config.normal_file = fname;
                config.normal_enabled = true;
            }
            if let Some(ref m) = evaluated.metalness {
                let fname = format!("{}_metalness.png", clean_name);
                let _ = m.save(export_dir.join(&fname));
                config.metalness_file = fname;
                config.metalness_enabled = true;
            }
            if let Some(ref s) = evaluated.smoothness {
                let fname = format!("{}_smoothness.png", clean_name);
                let _ = s.save(export_dir.join(&fname));
                config.smoothness_file = fname;
                config.smoothness_enabled = true;
            }
            if let Some(ref ao) = evaluated.occlusion {
                let fname = format!("{}_ao.png", clean_name);
                let _ = ao.save(export_dir.join(&fname));
                config.occlusion_file = fname;
                config.occlusion_enabled = true;
            }
            if let Some(ref em) = evaluated.emissive {
                let fname = format!("{}_emissive.png", clean_name);
                let _ = em.save(export_dir.join(&fname));
                config.emissive_file = fname;
                config.emissive_enabled = true;
            }
            if let Some(ref ca) = evaluated.custom_a {
                let fname = format!("{}_custom_a.png", clean_name);
                let _ = ca.save(export_dir.join(&fname));
                config.custom_a_file = fname;
                config.custom_a_enabled = true;
                config.custom_enabled = true;
            }
            if let Some(ref cb) = evaluated.custom_b {
                let fname = format!("{}_custom_b.png", clean_name);
                let _ = cb.save(export_dir.join(&fname));
                config.custom_b_file = fname;
                config.custom_b_enabled = true;
                config.custom_enabled = true;
            }

            let fx_content = config.generate_fx_code();
            let fx_path = export_dir.join(format!("{}.fx", clean_name));
            let _ = std::fs::write(fx_path, fx_content);

            exported_count += 1;
        }

        self.status_message = format!("Exported {} PMX subset materials to {:?}", exported_count, export_dir);
    }

    /// Automatically sets up full ShaderMap PBR network for the active graph from an image file.
    pub fn auto_generate_pbr_from_image(&mut self, file_path: &str) {
        let mut snarl = Snarl::new();

        let img_node = snarl.insert_node(
            egui::pos2(80.0, 100.0),
            MaterialNode::ImageInput {
                file_path: file_path.to_string(),
                is_srgb: true,
                cached_image: None,
            },
        );

        let height_node = snarl.insert_node(
            egui::pos2(320.0, 100.0),
            MaterialNode::HeightGenerator {
                contrast: 1.5,
                brightness: 0.0,
                invert: false,
            },
        );

        let normal_node = snarl.insert_node(
            egui::pos2(560.0, 40.0),
            MaterialNode::NormalGenerator {
                scale: 2.0,
                filter: NormalFilter::Scharr,
                orientation: NormalOrientation::DirectX,
            },
        );

        let ao_node = snarl.insert_node(
            egui::pos2(560.0, 200.0),
            MaterialNode::AOGenerator {
                radius: 16,
                samples: 16,
                intensity: 1.2,
                bias: 0.05,
            },
        );

        let rough_node = snarl.insert_node(
            egui::pos2(560.0, 360.0),
            MaterialNode::RoughnessGenerator {
                invert: false,
                contrast: 1.2,
                min_val: 0.1,
                max_val: 0.9,
            },
        );

        let metal_node = snarl.insert_node(
            egui::pos2(560.0, 520.0),
            MaterialNode::MetalnessGenerator {
                threshold: 0.5,
                falloff: 0.2,
                detect_metals: true,
                invert: false,
            },
        );

        let out_node = snarl.insert_node(
            egui::pos2(880.0, 100.0),
            MaterialNode::RayMaterialOutput {
                material_name: "autogenerated_pbr".to_string(),
                shading_model: crate::graph::node::ShadingModel::Default,
                albedo_color: [1.0, 1.0, 1.0],
                albedo_loop: [1.0, 1.0],
                normal_scale: 1.0,
                normal_loop: 1.0,
                smoothness_val: 0.5,
                is_roughness_mode: false,
                metalness_val: 0.0,
                specular_color: [0.5, 0.5, 0.5],
                occlusion_val: 1.0,
                parallax_scale: 0.05,
                emissive_color: [1.0, 1.0, 1.0],
                emissive_intensity: 1.0,
                emissive_blink_mode: 0,
                emissive_blink_freq: [1.0, 1.0, 1.0],
                custom_a_val: 0.0,
                custom_b_color: [0.0, 0.0, 0.0],
                hex_tiling_enable: false,
                hex_tiling_rotation: 1.0,
                hex_tiling_contrast: 0.6,
                hex_tiling_sharpness: 7.0,
                hashed_alpha_enable: false,
                hashed_alpha_scale: 1.0,
                detail_map_enable: false,
                detail_normal_scale: 1.0,
                detail_normal_loop: 20.0,
                detail_fade_distance: 15.0,
            },
        );

        // Connections
        snarl.connect(OutPinId { node: img_node, output: 0 }, InPinId { node: height_node, input: 0 });
        snarl.connect(OutPinId { node: img_node, output: 0 }, InPinId { node: metal_node, input: 0 });
        snarl.connect(OutPinId { node: height_node, output: 0 }, InPinId { node: normal_node, input: 0 });
        snarl.connect(OutPinId { node: height_node, output: 0 }, InPinId { node: ao_node, input: 0 });
        snarl.connect(OutPinId { node: height_node, output: 0 }, InPinId { node: rough_node, input: 0 });

        snarl.connect(OutPinId { node: img_node, output: 0 }, InPinId { node: out_node, input: 0 });
        snarl.connect(OutPinId { node: normal_node, output: 0 }, InPinId { node: out_node, input: 3 });
        snarl.connect(OutPinId { node: rough_node, output: 0 }, InPinId { node: out_node, input: 4 });
        snarl.connect(OutPinId { node: metal_node, output: 0 }, InPinId { node: out_node, input: 5 });
        snarl.connect(OutPinId { node: ao_node, output: 0 }, InPinId { node: out_node, input: 7 });
        snarl.connect(OutPinId { node: height_node, output: 0 }, InPinId { node: out_node, input: 8 });

        if self.app_mode == AppMode::PmxStudio {
            if let Some(idx) = self.active_pmx_subset {
                if let Some(slot) = self.pmx_slots.get_mut(idx) {
                    slot.snarl = snarl;
                    slot.is_dirty = true;
                    slot.has_custom_material = true;
                }
            }
        } else {
            self.snarl = snarl;
            self.graph_dirty = true;
        }
    }

    /// Smooth, professional camera interaction for 3D viewports.
    /// - LMB drag: Orbit around target
    /// - MMB drag / Shift + LMB drag: Pan target in camera plane
    /// - RMB drag / Alt + LMB drag: Smooth zoom
    /// - Mouse wheel: Exponential zoom
    /// - Ctrl + LMB drag: Rotate light source
    pub fn handle_camera_interaction(&mut self, is_pmx: bool, resp: &egui::Response, ui: &egui::Ui) {
        let (camera, dirty) = if is_pmx {
            (&mut self.pmx_camera, &mut self.pmx_preview_dirty)
        } else {
            (&mut self.camera, &mut self.preview_dirty)
        };

        if resp.dragged() {
            let delta = resp.drag_delta();
            let is_shift = ui.input(|i| i.modifiers.shift);
            let is_ctrl = ui.input(|i| i.modifiers.ctrl);
            let is_alt = ui.input(|i| i.modifiers.alt);
            let is_middle = ui.input(|i| i.pointer.middle_down());
            let is_secondary = ui.input(|i| i.pointer.secondary_down());

            if is_ctrl {
                camera.light_yaw += delta.x * 0.01;
                camera.light_pitch = (camera.light_pitch - delta.y * 0.01).clamp(-1.4, 1.4);
                *dirty = true;
            } else if is_middle || (is_shift && !is_secondary) {
                let dist = camera.distance.max(0.05);
                let cam_pos = glam::Vec3::new(
                    camera.target.x + dist * camera.pitch.cos() * camera.yaw.sin(),
                    camera.target.y + dist * camera.pitch.sin(),
                    camera.target.z - dist * camera.pitch.cos() * camera.yaw.cos(),
                );
                let cam_forward = (camera.target - cam_pos).normalize_or_zero();
                let cam_right = if cam_forward.y.abs() > 0.99 {
                    glam::Vec3::X
                } else {
                    glam::Vec3::Y.cross(cam_forward).normalize_or_zero()
                };
                let cam_up = cam_forward.cross(cam_right).normalize_or_zero();

                let pan_speed = (camera.distance * 0.0018).max(0.005);
                camera.target += cam_right * (-delta.x * pan_speed) + cam_up * (delta.y * pan_speed);
                *dirty = true;
            } else if is_secondary || is_alt {
                let zoom_speed = (camera.distance * 0.005).max(0.01);
                camera.distance = (camera.distance - delta.y * zoom_speed).max(0.05);
                *dirty = true;
            } else {
                camera.yaw += delta.x * 0.008;
                camera.pitch = (camera.pitch - delta.y * 0.008).clamp(-1.5, 1.5);
                *dirty = true;
            }
            self.is_interacting = true;
        }

        if resp.drag_stopped() {
            self.is_interacting = false;
            *dirty = true;
        }

        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > 0.1 {
                let zoom_factor = (1.0 - scroll * 0.002).clamp(0.5, 1.8);
                camera.distance = (camera.distance * zoom_factor).max(0.05);
                *dirty = true;
            }
        }
    }
}

impl eframe::App for MaterialEditorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // 1. Receive background evaluation results
        let mut got_single_result = false;
        let mut got_pmx_result = false;
        while let Ok(res) = self.eval_rx.try_recv() {
            self.last_eval_time_ms = res.eval_time_ms;
            self.is_evaluating = false;
            if let Some(slot_idx) = res.slot_index {
                if let Some(slot) = self.pmx_slots.get_mut(slot_idx) {
                    slot.evaluated = Some(res.evaluated);
                    slot.is_dirty = false;
                    got_pmx_result = true;
                }
            } else {
                self.evaluated = res.evaluated;
                got_single_result = true;
            }
        }

        if got_single_result {
            self.update_channel_textures(&ctx);
            self.preview_dirty = true;
            self.full_map_dirty = true;
            ctx.request_repaint();
        }
        if got_pmx_result {
            self.update_channel_textures(&ctx);
            self.pmx_preview_dirty = true;
            self.full_map_dirty = true;
            ctx.request_repaint();
        }

        if self.full_map_dirty {
            self.update_full_map_texture(&ctx);
            self.full_map_dirty = false;
        }

        // 2. Process graph changes
        if self.viewer.needs_rebuild {
            self.graph_dirty = true;
            self.viewer.needs_rebuild = false;
        }

        if self.app_mode == AppMode::SingleMaterial {
            if self.graph_dirty {
                if self.is_evaluating {
                    self.pending_eval = true;
                } else {
                    let req = EvalRequest {
                        snarl: self.snarl.clone(),
                        working_resolution: self.working_resolution,
                        slot_index: None,
                    };
                    let _ = self.eval_tx.send(req);
                    self.is_evaluating = true;
                    self.graph_dirty = false;
                }
            } else if !self.is_evaluating && self.pending_eval {
                let req = EvalRequest {
                    snarl: self.snarl.clone(),
                    working_resolution: self.working_resolution,
                    slot_index: None,
                };
                let _ = self.eval_tx.send(req);
                self.is_evaluating = true;
                self.pending_eval = false;
            }
        } else if let Some(active_idx) = self.active_pmx_subset {
            if let Some(slot) = self.pmx_slots.get_mut(active_idx) {
                if slot.is_dirty || self.graph_dirty {
                    if self.is_evaluating {
                        self.pending_eval = true;
                    } else {
                        let req = EvalRequest {
                            snarl: slot.snarl.clone(),
                            working_resolution: self.working_resolution,
                            slot_index: Some(active_idx),
                        };
                        let _ = self.eval_tx.send(req);
                        self.is_evaluating = true;
                        slot.is_dirty = false;
                        self.graph_dirty = false;
                    }
                } else if !self.is_evaluating && self.pending_eval {
                    let req = EvalRequest {
                        snarl: slot.snarl.clone(),
                        working_resolution: self.working_resolution,
                        slot_index: Some(active_idx),
                    };
                    let _ = self.eval_tx.send(req);
                    self.is_evaluating = true;
                    self.pending_eval = false;
                }
            }
        }

        // 3. Process viewport preview render
        if self.preview_dirty || self.preview_texture.is_none() {
            self.render_preview(&ctx);
            self.preview_dirty = false;
        }

        if (self.pmx_preview_dirty || self.pmx_preview_texture.is_none()) && self.pmx_model.is_some() {
            self.update_pmx_preview(&ctx);
        }

        // Top Menu Bar
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // Workspace Mode Selector
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.app_mode == AppMode::SingleMaterial, "🎨 Single Material Editor").clicked() {
                        self.app_mode = AppMode::SingleMaterial;
                    }
                    if ui.selectable_label(self.app_mode == AppMode::PmxStudio, "💃 PMX Model Studio").clicked() {
                        self.app_mode = AppMode::PmxStudio;
                        if self.pmx_model.is_some() {
                            self.pmx_preview_dirty = true;
                        }
                    }
                });

                ui.separator();

                ui.menu_button("File", |ui| {
                    if ui.button("📂 Open PMX Model...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PMX Model (*.pmx)", &["pmx"])
                            .pick_file()
                        {
                            self.load_pmx_file(path, &ctx);
                        }
                        ui.close();
                    }
                    if self.app_mode == AppMode::PmxStudio && ui.button("💾 Export All PMX Materials...").clicked() {
                        self.export_all_pmx_materials();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("New Graph").clicked() {
                        self.load_default_preset();
                        ui.close();
                    }
                    if ui.button("Save Graph (.rfmat)...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("ReForge Material", &["rfmat", "json"])
                            .save_file()
                        {
                            if let Ok(json) = serde_json::to_string_pretty(&self.snarl) {
                                let _ = std::fs::write(path, json);
                            }
                        }
                        ui.close();
                    }
                    if ui.button("Load Graph (.rfmat)...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("ReForge Material", &["rfmat", "json"])
                            .pick_file()
                        {
                            if let Ok(json) = std::fs::read_to_string(path) {
                                if let Ok(loaded) = serde_json::from_str(&json) {
                                    self.snarl = loaded;
                                    self.graph_dirty = true;
                                }
                            }
                        }
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("⚡ Export Ray-MMD Material (.fx + Maps)").clicked() {
                        self.export_ray_material();
                        ui.close();
                    }
                });

                ui.menu_button("Presets", |ui| {
                    if ui.button("Standard PBR (Stone / Plastic)").clicked() {
                        self.load_default_preset();
                        ui.close();
                    }
                    if ui.button("Polished Chrome / Metal").clicked() {
                        self.load_default_preset();
                        for node in self.snarl.nodes_mut() {
                            if let MaterialNode::RayMaterialOutput { metalness_val, smoothness_val, .. } = node {
                                *metalness_val = 1.0;
                                *smoothness_val = 0.95;
                            }
                        }
                        self.graph_dirty = true;
                        ui.close();
                    }
                    if ui.button("Sci-Fi Hex Glowing Emissive").clicked() {
                        self.load_default_preset();
                        for node in self.snarl.nodes_mut() {
                            if let MaterialNode::RayMaterialOutput {
                                hex_tiling_enable,
                                emissive_color,
                                emissive_intensity,
                                ..
                            } = node {
                                *hex_tiling_enable = true;
                                *emissive_color = [0.1, 0.8, 1.0];
                                *emissive_intensity = 3.5;
                            }
                        }
                        self.graph_dirty = true;
                        ui.close();
                    }
                });

                ui.separator();
                if ui.button("⚡ Export to Ray-MMD").clicked() {
                    self.export_ray_material();
                }

                ui.separator();
                ui.label("Working Res:");
                let old_res = self.working_resolution;
                egui::ComboBox::from_id_salt("working_res_combo")
                    .selected_text(match self.working_resolution {
                        256 => "256 (Ultra Fast)",
                        512 => "512 (Fast / Recommended)",
                        1024 => "1024 (Balanced)",
                        0 => "Full Source",
                        _ => "Custom",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.working_resolution, 256, "256 (Ultra Fast)");
                        ui.selectable_value(&mut self.working_resolution, 512, "512 (Fast / Recommended)");
                        ui.selectable_value(&mut self.working_resolution, 1024, "1024 (Balanced)");
                        ui.selectable_value(&mut self.working_resolution, 0, "Full Source");
                    });
                if old_res != self.working_resolution {
                    self.graph_dirty = true;
                }

                ui.separator();
                if self.is_evaluating {
                    ui.colored_label(Color32::from_rgb(255, 200, 50), "⏳ Evaluating...");
                    ctx.request_repaint();
                } else {
                    ui.colored_label(
                        Color32::from_rgb(80, 220, 100),
                        format!("⚡ {:.1}ms", self.last_eval_time_ms),
                    );
                }
            });
        });

        // Bottom Status & Map Gallery Panel
        egui::Panel::bottom("bottom_panel")
            .resizable(true)
            .default_size(150.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Map Gallery:");
                    if ui.selectable_label(self.active_tab == MapViewTab::All, "All Maps").clicked() {
                        self.active_tab = MapViewTab::All;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Albedo, "Albedo").clicked() {
                        self.active_tab = MapViewTab::Albedo;
                        self.selected_map_name = "Albedo".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Normal, "Normal").clicked() {
                        self.active_tab = MapViewTab::Normal;
                        self.selected_map_name = "Normal".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Smoothness, "Roughness").clicked() {
                        self.active_tab = MapViewTab::Smoothness;
                        self.selected_map_name = "Smoothness".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Metalness, "Metalness").clicked() {
                        self.active_tab = MapViewTab::Metalness;
                        self.selected_map_name = "Metalness".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Occlusion, "AO").clicked() {
                        self.active_tab = MapViewTab::Occlusion;
                        self.selected_map_name = "AO".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Parallax, "Height").clicked() {
                        self.active_tab = MapViewTab::Parallax;
                        self.selected_map_name = "Parallax".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::Emissive, "Emissive").clicked() {
                        self.active_tab = MapViewTab::Emissive;
                        self.selected_map_name = "Emissive".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::CustomA, "Custom A").clicked() {
                        self.active_tab = MapViewTab::CustomA;
                        self.selected_map_name = "Custom A".to_string();
                        self.full_map_dirty = true;
                    }
                    if ui.selectable_label(self.active_tab == MapViewTab::CustomB, "Custom B").clicked() {
                        self.active_tab = MapViewTab::CustomB;
                        self.selected_map_name = "Custom B".to_string();
                        self.full_map_dirty = true;
                    }

                    ui.separator();
                    if ui.button("🔍 2D Map Inspector").on_hover_text("Open dedicated floating 2D Texture Inspector window with zoom, pan, and pixel probe").clicked() {
                        self.show_map_inspector_window = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(&self.status_message);
                    });
                });

                ui.separator();

                let mut clicked_map = None;
                let mut double_clicked_map = None;
                let mut save_map = None;
                let mut switch_to_all = false;
                let mut view_2d_clicked = None;
                let mut inspect_window_clicked = None;

                let selected_map = self.selected_map_name.clone();
                let draw_map_card = |ui: &mut egui::Ui, name: &str, tex: Option<&TextureHandle>, clicked: &mut Option<String>, dbl_clicked: &mut Option<String>| {
                    let is_selected = selected_map == name;
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(name);
                            if is_selected {
                                ui.colored_label(Color32::from_rgb(80, 200, 255), "●");
                            }
                        });

                        let (rect, resp) = ui.allocate_exact_size(Vec2::new(96.0, 96.0), egui::Sense::click());
                        if let Some(t) = tex {
                            ui.painter().image(
                                t.id(),
                                rect,
                                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        } else {
                            ui.painter().rect_filled(rect, 4.0, Color32::from_rgb(30, 30, 35));
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "None",
                                egui::FontId::proportional(12.0),
                                Color32::GRAY,
                            );
                        }

                        if is_selected {
                            ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(2.0, Color32::from_rgb(80, 180, 255)), egui::StrokeKind::Outside);
                        } else if resp.hovered() {
                            ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.5, Color32::from_rgb(180, 180, 180)), egui::StrokeKind::Outside);
                        }

                        if resp.clicked() {
                            *clicked = Some(name.to_string());
                        }
                        if resp.double_clicked() {
                            *dbl_clicked = Some(name.to_string());
                        }
                    });
                };

                if self.active_tab == MapViewTab::All {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for name in &["Albedo", "Normal", "Smoothness", "Metalness", "AO", "Parallax", "Emissive", "Custom A", "Custom B"] {
                                draw_map_card(ui, name, self.channel_textures.get(*name), &mut clicked_map, &mut double_clicked_map);
                            }
                        });
                    });
                } else {
                    let active_name = self.active_tab.map_name().unwrap_or("Albedo");
                    let has_info = self.get_evaluated_map(active_name).map(|img| (img.width(), img.height()));
                    let active_tex = self.channel_textures.get(active_name);
                    ui.horizontal(|ui| {
                        draw_map_card(ui, active_name, active_tex, &mut clicked_map, &mut double_clicked_map);
                        ui.add_space(16.0);
                        ui.vertical(|ui| {
                            ui.heading(format!("Map Channel: {}", active_name));
                            if let Some((w, h)) = has_info {
                                ui.label(format!("Resolution: {} × {} px", w, h));
                                ui.label("Color Format: RGBA 8-bit (uncompressed)");
                            } else {
                                ui.colored_label(Color32::GRAY, "Channel not connected or empty");
                            }
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                if ui.button("🔍 View in 2D Viewport").clicked() {
                                    view_2d_clicked = Some(active_name.to_string());
                                }
                                if ui.button("🖼 Floating Inspector Window").clicked() {
                                    inspect_window_clicked = Some(active_name.to_string());
                                }
                                if ui.button("💾 Save Map to File...").clicked() {
                                    save_map = Some(active_name.to_string());
                                }
                                if ui.button("📋 Show All Maps").clicked() {
                                    switch_to_all = true;
                                }
                            });
                        });
                    });
                }

                if let Some(name) = clicked_map {
                    self.selected_map_name = name;
                    self.full_map_dirty = true;
                    self.viewport_mode = ViewportMode::Map2D;
                }
                if let Some(name) = double_clicked_map {
                    self.selected_map_name = name;
                    self.full_map_dirty = true;
                    self.show_map_inspector_window = true;
                }
                if let Some(name) = view_2d_clicked {
                    self.selected_map_name = name;
                    self.full_map_dirty = true;
                    self.viewport_mode = ViewportMode::Map2D;
                }
                if let Some(name) = inspect_window_clicked {
                    self.selected_map_name = name;
                    self.full_map_dirty = true;
                    self.show_map_inspector_window = true;
                }
                if let Some(name) = save_map {
                    self.save_single_map(&name);
                }
                if switch_to_all {
                    self.active_tab = MapViewTab::All;
                }
            });

        // Left Panel: Node Palette (Single Mode) or PMX Subsets Manager (PMX Mode)
        egui::Panel::left("left_panel")
            .resizable(true)
            .default_size(240.0)
            .show(ui, |ui| {
                if self.app_mode == AppMode::SingleMaterial {
                    ui.heading("Node Palette");
                    ui.separator();

                    ui.label(egui::RichText::new("⚡ QUICK PBR TOOLS").strong());
                    if ui.button("⚡ Auto PBR from Image...").on_hover_text("Automatically generates full ShaderMap PBR material (Height, Normal, Roughness, Metalness, AO) from a texture").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "tga", "bmp"])
                            .pick_file()
                        {
                            self.auto_generate_pbr_from_image(&path.to_string_lossy());
                        }
                    }
                    if ui.button("🪙 Add Metalness Generator").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(120.0, 150.0),
                            MaterialNode::MetalnessGenerator {
                                threshold: 0.5,
                                falloff: 0.2,
                                detect_metals: true,
                                invert: false,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("💡 Add Emissive Mask Generator").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(120.0, 150.0),
                            MaterialNode::EmissiveGenerator {
                                min_lum: 0.5,
                                max_lum: 1.0,
                                use_hue_filter: false,
                                target_hue: 180.0,
                                hue_tolerance: 45.0,
                                tint_color: [1.0, 1.0, 1.0],
                                intensity: 2.0,
                                invert: false,
                            },
                        );
                        self.graph_dirty = true;
                    }

                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("INPUTS").strong());
                    if ui.button("➕ Texture Image").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::ImageInput {
                                file_path: String::new(),
                                is_srgb: true,
                                cached_image: None,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("➕ Color Value").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::ColorInput {
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("➕ Float Value").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::FloatInput {
                                value: 1.0,
                                min: 0.0,
                                max: 1.0,
                            },
                        );
                        self.graph_dirty = true;
                    }

                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("SHADERMAP GENERATORS").strong());
                    if ui.button("⚡ Height Generator").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::HeightGenerator {
                                contrast: 1.0,
                                brightness: 0.0,
                                invert: false,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("⚡ Normal Generator").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::NormalGenerator {
                                scale: 1.0,
                                filter: NormalFilter::Scharr,
                                orientation: NormalOrientation::DirectX,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("⚡ Ambient Occlusion (AO)").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::AOGenerator {
                                radius: 16,
                                samples: 16,
                                intensity: 1.0,
                                bias: 0.05,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("⚡ Curvature / Cavity").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::CurvatureGenerator {
                                radius: 2,
                                intensity: 2.0,
                                mode: CurvatureMode::Full,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("⚡ Roughness Remap").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::RoughnessGenerator {
                                invert: false,
                                contrast: 1.0,
                                min_val: 0.0,
                                max_val: 1.0,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("⚡ Custom Map Generator").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::CustomMapGenerator {
                                model: crate::graph::node::ShadingModel::Skin,
                                param_a: 1.0,
                                param_b_color: [1.0, 0.4, 0.25],
                                invert_a: false,
                                aniso_radial: false,
                            },
                        );
                        self.graph_dirty = true;
                    }

                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("FILTERS & COMBINERS").strong());
                    if ui.button("🔀 Normal Blend (RNM)").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::NormalBlend {
                                detail_scale: 1.0,
                                detail_tile: 10.0,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("📦 Channel Packer (RGBA)").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::ChannelPacker {
                                default_r: 128,
                                default_g: 0,
                                default_b: 255,
                                default_a: 255,
                            },
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("✂ Channel Splitter").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::ChannelSplitter,
                        );
                        self.graph_dirty = true;
                    }
                    if ui.button("🎨 Color Blend").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::ColorBlend {
                                mode: crate::graph::node::BlendMode::Mix,
                                factor: 0.5,
                            },
                        );
                        self.graph_dirty = true;
                    }

                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("OUTPUTS").strong());
                    if ui.button("🎯 Ray-MMD Master Output").clicked() {
                        self.snarl.insert_node(
                            egui::pos2(100.0, 100.0),
                            MaterialNode::RayMaterialOutput {
                                material_name: "reforge_material".to_string(),
                                shading_model: crate::graph::node::ShadingModel::Default,
                                albedo_color: [1.0, 1.0, 1.0],
                                albedo_loop: [1.0, 1.0],
                                normal_scale: 1.0,
                                normal_loop: 1.0,
                                smoothness_val: 0.5,
                                is_roughness_mode: false,
                                metalness_val: 0.0,
                                specular_color: [0.5, 0.5, 0.5],
                                occlusion_val: 1.0,
                                parallax_scale: 0.05,
                                emissive_color: [1.0, 1.0, 1.0],
                                emissive_intensity: 1.0,
                                emissive_blink_mode: 0,
                                emissive_blink_freq: [1.0, 1.0, 1.0],
                                custom_a_val: 0.0,
                                custom_b_color: [0.0, 0.0, 0.0],
                                hex_tiling_enable: false,
                                hex_tiling_rotation: 1.0,
                                hex_tiling_contrast: 0.6,
                                hex_tiling_sharpness: 7.0,
                                hashed_alpha_enable: false,
                                hashed_alpha_scale: 1.0,
                                detail_map_enable: false,
                                detail_normal_scale: 1.0,
                                detail_normal_loop: 20.0,
                                detail_fade_distance: 15.0,
                            },
                        );
                        self.graph_dirty = true;
                    }
                } else {
                    // PMX Model Studio Left Panel: Subsets Manager
                    ui.heading("PMX Model Subsets");
                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("📂 Open PMX...").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("PMX Model (*.pmx)", &["pmx"])
                                .pick_file()
                            {
                                self.load_pmx_file(path, &ctx);
                            }
                        }
                        if self.pmx_model.is_some() && ui.button("💾 Export All").clicked() {
                            self.export_all_pmx_materials();
                        }
                    });

                    if self.pmx_model.is_some() {
                        let (name_str, geo_str) = if let Some(ref model) = self.pmx_model {
                            (
                                format!("Model: {}", if !model.name_universal.is_empty() { &model.name_universal } else { &model.name_local }),
                                format!("Geometry: {} verts | {} tris", model.vertices.len(), model.indices.len() / 3),
                            )
                        } else {
                            (String::new(), String::new())
                        };

                        ui.small(name_str);
                        ui.small(geo_str);

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Filter:");
                            ui.text_edit_singleline(&mut self.pmx_search_filter);
                        });

                        ui.separator();

                        let mut auto_pbr_path = None;
                        if let Some(active_idx) = self.active_pmx_subset {
                            ui.horizontal(|ui| {
                                if ui.button("⚡ Auto PBR").on_hover_text("Generate PBR material for selected subset from its base PMX texture").clicked() {
                                    if let Some(ref model) = self.pmx_model {
                                        if let Some(ref tex_path) = model.subsets.get(active_idx).and_then(|s| s.absolute_texture_path.as_ref()) {
                                            auto_pbr_path = Some(tex_path.to_string_lossy().to_string());
                                        }
                                    }
                                }
                                if ui.button("🎯 Focus Subset").on_hover_text("Center camera directly on this material subset").clicked() {
                                    if let Some(ref model) = self.pmx_model {
                                        if let Some(sub) = model.subsets.get(active_idx) {
                                            let mut s_min = glam::Vec3::splat(f32::MAX);
                                            let mut s_max = glam::Vec3::splat(f32::MIN);
                                            for t in 0..(sub.index_count / 3) {
                                                let base = sub.index_start + t * 3;
                                                for k in 0..3 {
                                                    if let Some(&v_idx) = model.indices.get(base + k) {
                                                        if let Some(v) = model.vertices.get(v_idx as usize) {
                                                            s_min = s_min.min(v.position);
                                                            s_max = s_max.max(v.position);
                                                        }
                                                    }
                                                }
                                            }
                                            if s_min.x <= s_max.x {
                                                let center = (s_min + s_max) * 0.5;
                                                let size = (s_max - s_min).length();
                                                self.pmx_camera.target = center;
                                                self.pmx_camera.distance = (size * 1.2).clamp(3.0, 90.0);
                                                self.pmx_preview_dirty = true;
                                            }
                                        }
                                    }
                                }
                            });
                            ui.separator();
                        }

                        if let Some(path) = auto_pbr_path {
                            self.auto_generate_pbr_from_image(&path);
                        }

                        let search = self.pmx_search_filter.to_lowercase();
                        let mut toggled_subset: Option<(usize, bool)> = None;
                        let mut selected_subset: Option<usize> = None;

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (idx, slot) in self.pmx_slots.iter_mut().enumerate() {
                                if !search.is_empty() && !slot.name.to_lowercase().contains(&search) {
                                    continue;
                                }

                                let is_active = self.active_pmx_subset == Some(idx);
                                ui.horizontal(|ui| {
                                    if ui.checkbox(&mut slot.is_visible, "").changed() {
                                        toggled_subset = Some((idx, slot.is_visible));
                                    }

                                    let badge = if slot.has_custom_material { "●" } else { "○" };
                                    let badge_color = if slot.has_custom_material { Color32::from_rgb(80, 220, 100) } else { Color32::GRAY };

                                    ui.colored_label(badge_color, badge);

                                    let label_text = format!("[{:02}] {}", idx, slot.name);
                                    if ui.selectable_label(is_active, label_text).clicked() {
                                        selected_subset = Some(idx);
                                    }
                                });
                            }
                        });

                        if let Some((idx, visible)) = toggled_subset {
                            if let Some(ref mut model) = self.pmx_model {
                                if let Some(sub) = model.subsets.get_mut(idx) {
                                    sub.is_visible = visible;
                                }
                            }
                            self.pmx_preview_dirty = true;
                        }

                        if let Some(idx) = selected_subset {
                            self.active_pmx_subset = Some(idx);
                            self.pmx_preview_dirty = true;
                            self.full_map_dirty = true;
                        }
                    } else {
                        ui.add_space(20.0);
                        ui.label("No PMX model loaded.");
                        ui.label("Click 'Open PMX' to load a 3D model with all its subsets.");
                    }
                }
            });

        // Right Panel: 3D Material / PMX Model Viewport or 2D Texture Map Viewer
        egui::Panel::right("right_panel")
            .resizable(true)
            .default_size(380.0)
            .show(ui, |ui| {
                let is_fullscreen_pmx = self.app_mode == AppMode::PmxStudio && self.pmx_center_view_mode == 1;

                if is_fullscreen_pmx {
                    ui.heading("🖼 2D Texture Map Inspector");
                    ui.separator();
                    self.show_2d_map_viewer(ui);
                } else {
                    ui.horizontal(|ui| {
                        let pbr_label = if self.app_mode == AppMode::PmxStudio { "💃 3D Model" } else { "🌐 3D Viewport" };
                        if ui.selectable_label(self.viewport_mode == ViewportMode::Pbr3D, pbr_label).clicked() {
                            self.viewport_mode = ViewportMode::Pbr3D;
                        }
                        if ui.selectable_label(self.viewport_mode == ViewportMode::Map2D, "🖼 2D Texture Viewer").clicked() {
                            self.viewport_mode = ViewportMode::Map2D;
                            self.full_map_dirty = true;
                        }
                    });
                    ui.separator();

                    match self.viewport_mode {
                        ViewportMode::Pbr3D => {
                            if self.app_mode == AppMode::SingleMaterial {
                                ui.heading("3D Material Viewport");
                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.label("Mesh:");
                                    if ui.selectable_value(&mut self.camera.primitive, PreviewPrimitive::Sphere, "Sphere").changed() {
                                        self.preview_dirty = true;
                                    }
                                    if ui.selectable_value(&mut self.camera.primitive, PreviewPrimitive::Cube, "Cube").changed() {
                                        self.preview_dirty = true;
                                    }
                                    if ui.selectable_value(&mut self.camera.primitive, PreviewPrimitive::Plane, "Plane").changed() {
                                        self.preview_dirty = true;
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Display:");
                                    egui::ComboBox::from_id_salt("single_disp_mode")
                                        .selected_text(self.camera.display_mode.display_name())
                                        .show_ui(ui, |ui| {
                                            for mode in &[
                                                ViewportDisplayMode::FullPbr,
                                                ViewportDisplayMode::AlbedoOnly,
                                                ViewportDisplayMode::NormalOnly,
                                                ViewportDisplayMode::RoughnessOnly,
                                                ViewportDisplayMode::MetalnessOnly,
                                                ViewportDisplayMode::OcclusionOnly,
                                                ViewportDisplayMode::EmissiveOnly,
                                                ViewportDisplayMode::CustomAOnly,
                                                ViewportDisplayMode::CustomBOnly,
                                            ] {
                                                if ui.selectable_value(&mut self.camera.display_mode, *mode, mode.display_name()).changed() {
                                                    self.preview_dirty = true;
                                                }
                                            }
                                        });
                                });

                                if let Some(ref tex) = self.preview_texture {
                                    let avail_w = ui.available_width();
                                    let preview_size = Vec2::new(avail_w, avail_w);
                                    let resp = ui.image((tex.id(), preview_size)).interact(egui::Sense::drag());
                                    self.handle_camera_interaction(false, &resp, ui);
                                }

                                ui.small("LMB: Orbit | MMB / Shift+LMB: Pan | Scroll / RMB: Zoom | Ctrl+LMB: Light");
                                ui.separator();

                                ui.collapsing("Lighting & Environment", |ui| {
                                    if ui.add(egui::Slider::new(&mut self.camera.light_intensity, 0.0..=5.0).text("Key Intensity")).changed() {
                                        self.preview_dirty = true;
                                    }
                                    if ui.add(egui::Slider::new(&mut self.camera.light_yaw, 0.0..=std::f32::consts::TAU).text("Light Azimuth")).changed() {
                                        self.preview_dirty = true;
                                    }
                                    if ui.add(egui::Slider::new(&mut self.camera.light_pitch, -1.4..=1.4).text("Light Elevation")).changed() {
                                        self.preview_dirty = true;
                                    }
                                });

                                ui.separator();
                                if ui.button("💾 Export Ray-MMD Package (.fx + Maps)").clicked() {
                                    self.export_ray_material();
                                }
                            } else {
                                // PMX Model Studio 3D Viewport
                                ui.heading("PMX 3D Model Viewport");
                                ui.separator();

                                if self.pmx_model.is_some() {
                                    ui.horizontal(|ui| {
                                        ui.label("Display:");
                                        egui::ComboBox::from_id_salt("pmx_disp_mode")
                                            .selected_text(self.pmx_camera.display_mode.display_name())
                                            .show_ui(ui, |ui| {
                                                for mode in &[
                                                    ViewportDisplayMode::FullPbr,
                                                    ViewportDisplayMode::AlbedoOnly,
                                                    ViewportDisplayMode::NormalOnly,
                                                    ViewportDisplayMode::RoughnessOnly,
                                                    ViewportDisplayMode::MetalnessOnly,
                                                    ViewportDisplayMode::OcclusionOnly,
                                                    ViewportDisplayMode::EmissiveOnly,
                                                    ViewportDisplayMode::CustomAOnly,
                                                    ViewportDisplayMode::CustomBOnly,
                                                ] {
                                                    if ui.selectable_value(&mut self.pmx_camera.display_mode, *mode, mode.display_name()).changed() {
                                                        self.pmx_preview_dirty = true;
                                                    }
                                                }
                                            });
                                    });

                                    ui.horizontal(|ui| {
                                        if ui.checkbox(&mut self.solo_active_subset, "Solo").on_hover_text("Render only active material subset").changed() {
                                            self.pmx_preview_dirty = true;
                                        }
                                        if ui.button("🎯 Focus").on_hover_text("Center camera on model bounding center").clicked() {
                                            if let Some(ref m) = self.pmx_model {
                                                self.pmx_camera.target = m.center;
                                                self.pmx_camera.distance = (m.radius * 2.2).max(10.0);
                                                self.pmx_camera.pitch = 0.15;
                                                self.pmx_camera.yaw = 0.0;
                                                self.pmx_preview_dirty = true;
                                            }
                                        }
                                        if ui.button("🏠 Stage View").on_hover_text("Reset camera to standard MMD stage interior view (target: Y=8, dist: 65)").clicked() {
                                            self.pmx_camera.target = glam::Vec3::new(0.0, 8.0, 10.0);
                                            self.pmx_camera.distance = 65.0;
                                            self.pmx_camera.pitch = 0.18;
                                            self.pmx_camera.yaw = 0.0;
                                            self.pmx_preview_dirty = true;
                                        }
                                    });

                                    let avail_w = ui.available_width();
                                    let preview_size = Vec2::new(avail_w, avail_w);
                                    if (preview_size.x - self.pmx_viewport_size.x).abs() > 2.0 || (preview_size.y - self.pmx_viewport_size.y).abs() > 2.0 {
                                        self.pmx_viewport_size = preview_size;
                                        self.pmx_preview_dirty = true;
                                    }
                                    if let Some(ref tex) = self.pmx_preview_texture {
                                        let resp = ui.image((tex.id(), preview_size)).interact(egui::Sense::drag());
                                        self.handle_camera_interaction(true, &resp, ui);
                                    }

                                    ui.small("LMB: Orbit | MMB / Shift+LMB: Pan | Scroll / RMB: Zoom | Ctrl+LMB: Light");
                                    ui.separator();

                                    ui.collapsing("Lighting & Environment", |ui| {
                                        if ui.add(egui::Slider::new(&mut self.pmx_camera.light_intensity, 0.0..=5.0).text("Key Intensity")).changed() {
                                            self.pmx_preview_dirty = true;
                                        }
                                        if ui.add(egui::Slider::new(&mut self.pmx_camera.light_yaw, 0.0..=std::f32::consts::TAU).text("Light Azimuth")).changed() {
                                            self.pmx_preview_dirty = true;
                                        }
                                        if ui.add(egui::Slider::new(&mut self.pmx_camera.light_pitch, -1.4..=1.4).text("Light Elevation")).changed() {
                                            self.pmx_preview_dirty = true;
                                        }
                                    });

                                    ui.separator();
                                    if ui.button("💾 Export All PMX Materials...").clicked() {
                                        self.export_all_pmx_materials();
                                    }
                                } else {
                                    ui.add_space(20.0);
                                    if ui.button("📂 Open PMX Model...").clicked() {
                                        if let Some(path) = rfd::FileDialog::new()
                                            .add_filter("PMX Model (*.pmx)", &["pmx"])
                                            .pick_file()
                                        {
                                            self.load_pmx_file(path, &ctx);
                                        }
                                    }
                                }
                            }
                        }
                        ViewportMode::Map2D => {
                            self.show_2d_map_viewer(ui);
                        }
                    }
                }
            });

        // Center Panel: Node Graph Canvas (Single Mode or PMX Active Subset Graph)
        egui::CentralPanel::default().show(ui, |ui| {
            if self.app_mode == AppMode::SingleMaterial {
                SnarlWidget::new().show(&mut self.snarl, &mut self.viewer, ui);

                if self.viewer.needs_rebuild {
                    self.graph_dirty = true;
                    ui.ctx().request_repaint();
                }

                if !self.viewer.nodes_to_remove.is_empty() {
                    for node_id in self.viewer.nodes_to_remove.drain(..) {
                        self.snarl.remove_node(node_id);
                    }
                    self.graph_dirty = true;
                    ui.ctx().request_repaint();
                }
            } else {
                // PMX Studio Center Panel
                ui.horizontal(|ui| {
                    if ui.selectable_value(&mut self.pmx_center_view_mode, 0, "🔀 Active Subset Material Graph").clicked() {
                        self.full_map_dirty = true;
                        self.pmx_preview_dirty = true;
                    }
                    if ui.selectable_value(&mut self.pmx_center_view_mode, 1, "💃 Fullscreen 3D Model").clicked() {
                        self.viewport_mode = ViewportMode::Map2D;
                        self.full_map_dirty = true;
                        self.pmx_preview_dirty = true;
                    }

                    if self.pmx_center_view_mode == 1 {
                        ui.separator();
                        if ui.button("🎯 Focus").on_hover_text("Center camera on model bounding center").clicked() {
                            if let Some(ref m) = self.pmx_model {
                                self.pmx_camera.target = m.center;
                                self.pmx_camera.distance = (m.radius * 2.2).max(10.0);
                                self.pmx_camera.pitch = 0.15;
                                self.pmx_camera.yaw = 0.0;
                                self.pmx_preview_dirty = true;
                            }
                        }
                        if ui.button("🏠 Stage View").on_hover_text("Reset camera to standard MMD stage interior view (target: Y=8, dist: 65)").clicked() {
                            self.pmx_camera.target = glam::Vec3::new(0.0, 8.0, 10.0);
                            self.pmx_camera.distance = 65.0;
                            self.pmx_camera.pitch = 0.18;
                            self.pmx_camera.yaw = 0.0;
                            self.pmx_preview_dirty = true;
                        }
                    }

                    if let Some(active_idx) = self.active_pmx_subset {
                        if let Some(slot) = self.pmx_slots.get(active_idx) {
                            ui.separator();
                            ui.colored_label(Color32::from_rgb(80, 200, 255), format!("Active Subset: [{:02}] {}", active_idx, slot.name));
                        }
                    }
                });
                ui.separator();

                if self.pmx_center_view_mode == 0 {
                    if let Some(active_idx) = self.active_pmx_subset {
                        if let Some(slot) = self.pmx_slots.get_mut(active_idx) {
                            SnarlWidget::new().show(&mut slot.snarl, &mut self.viewer, ui);

                            if self.viewer.needs_rebuild {
                                slot.is_dirty = true;
                                slot.has_custom_material = true;
                                self.graph_dirty = true;
                                self.pmx_preview_dirty = true;
                                self.viewer.needs_rebuild = false;
                                ui.ctx().request_repaint();
                            }

                            if !self.viewer.nodes_to_remove.is_empty() {
                                for node_id in self.viewer.nodes_to_remove.drain(..) {
                                    slot.snarl.remove_node(node_id);
                                }
                                slot.is_dirty = true;
                                slot.has_custom_material = true;
                                self.graph_dirty = true;
                                self.pmx_preview_dirty = true;
                                ui.ctx().request_repaint();
                            }
                        }
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Select a material subset from the left panel to edit its graph.");
                        });
                    }
                } else {
                    // Fullscreen 3D Model in center panel
                    let avail = ui.available_size();
                    if avail.x > 20.0 && avail.y > 20.0 {
                        if (avail.x - self.pmx_viewport_size.x).abs() > 2.0 || (avail.y - self.pmx_viewport_size.y).abs() > 2.0 {
                            self.pmx_viewport_size = avail;
                            self.pmx_preview_dirty = true;
                        }
                    }
                    if let Some(ref tex) = self.pmx_preview_texture {
                        let resp = ui.image((tex.id(), avail)).interact(egui::Sense::drag());
                        self.handle_camera_interaction(true, &resp, ui);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("No model loaded. Open a PMX model to preview.");
                        });
                    }
                }
            }
        });

        // Floating Texture Map Inspector Window
        if self.show_map_inspector_window {
            let mut open = true;
            egui::Window::new(format!("🖼 Texture Map Inspector — {}", self.selected_map_name))
                .open(&mut open)
                .default_size(Vec2::new(750.0, 600.0))
                .resizable(true)
                .show(ui.ctx(), |ui| {
                    self.show_2d_map_viewer(ui);
                });
            if !open {
                self.show_map_inspector_window = false;
            }
        }
    }
}
