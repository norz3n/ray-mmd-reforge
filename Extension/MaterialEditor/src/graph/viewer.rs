//! Snarl viewer implementation for drawing and interacting with material nodes in egui.

use egui::{Color32, DragValue, Slider, Ui};
use egui_snarl::ui::{PinInfo, PinShape, SnarlPin, SnarlStyle, SnarlViewer};
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use crate::graph::node::{BlendMode, MaterialNode, PinType};
use crate::image_proc::{CurvatureMode, NormalFilter, NormalOrientation};
use egui_phosphor::regular as icons;

pub fn create_blender_snarl_style() -> SnarlStyle {
    let mut style = SnarlStyle::new();
    style.collapsible = Some(true);
    style.wire_width = Some(2.2);
    style.pin_size = Some(5.0);
    style.header_drag_space = Some(egui::vec2(8.0, 0.0));
    style.node_frame = Some(egui::Frame {
        inner_margin: egui::Margin::symmetric(7, 6),
        corner_radius: egui::CornerRadius::same(7),
        fill: Color32::from_rgb(26, 28, 32),
        stroke: egui::Stroke::new(1.0, Color32::from_rgb(45, 48, 55)),
        ..Default::default()
    });
    style.header_frame = Some(egui::Frame {
        inner_margin: egui::Margin::symmetric(7, 4),
        corner_radius: egui::CornerRadius {
            nw: 6,
            ne: 6,
            sw: 0,
            se: 0,
        },
        fill: Color32::from_rgb(32, 34, 40),
        stroke: egui::Stroke::NONE,
        ..Default::default()
    });
    style
}

/// Returns pin styling and shape based on data type.
pub fn pin_info_for_type(pin_type: PinType) -> PinInfo {
    match pin_type {
        // Yellow: Color / RGBA
        PinType::Rgba => PinInfo::circle()
            .with_shape(PinShape::Circle)
            .with_fill(Color32::from_rgb(232, 184, 56))
            .with_wire_color(Color32::from_rgb(232, 184, 56)),
        // Light Gray: Grayscale / Value
        PinType::Grayscale => PinInfo::circle()
            .with_shape(PinShape::Circle)
            .with_fill(Color32::from_rgb(165, 168, 175))
            .with_wire_color(Color32::from_rgb(165, 168, 175)),
        // Triangle Gray: Float Scalar Value
        PinType::Float => PinInfo::circle()
            .with_shape(PinShape::Triangle)
            .with_fill(Color32::from_rgb(165, 168, 175))
            .with_wire_color(Color32::from_rgb(165, 168, 175)),
        // Purple: Vector / Normal 3D
        PinType::Vector => PinInfo::circle()
            .with_shape(PinShape::Circle)
            .with_fill(Color32::from_rgb(120, 95, 220))
            .with_wire_color(Color32::from_rgb(120, 95, 220)),
    }
}

/// The viewer state for egui-snarl.
pub struct MaterialSnarlViewer {
    /// Flag indicating the graph was modified and requires re-evaluation.
    pub needs_rebuild: bool,
    /// List of nodes queued for deletion after frame rendering.
    pub nodes_to_remove: Vec<NodeId>,
    /// Live node thumbnail textures for immediate in-node feedback.
    pub node_thumbnails: std::collections::HashMap<NodeId, egui::TextureHandle>,
    /// Snapshots captured on wire connections/disconnections for undo history.
    pub undo_snapshots: Vec<Snarl<MaterialNode>>,
}

impl MaterialSnarlViewer {
    pub fn new() -> Self {
        Self {
            needs_rebuild: true,
            nodes_to_remove: Vec::new(),
            node_thumbnails: std::collections::HashMap::new(),
            undo_snapshots: Vec::new(),
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
        let cat_color = snarl[node].category_color();
        let title = self.title(&snarl[node]);
        let is_master = matches!(snarl[node], MaterialNode::RayMaterialOutput { .. });

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);

            // Blender colored category pill
            let (rect, _response) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, egui::CornerRadius::same(2), cat_color);

            ui.add(egui::Label::new(
                egui::RichText::new(title)
                    .color(Color32::from_rgb(230, 233, 240))
                    .size(12.5)
                    .strong(),
            ));

            if !is_master {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button(egui::RichText::new(icons::X).size(8.5).color(Color32::from_rgb(180, 80, 80)))
                        .on_hover_text("Delete node")
                        .clicked()
                    {
                        self.nodes_to_remove.push(node);
                        self.needs_rebuild = true;
                    }
                });
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
            if ui.button(format!("{} Delete Node", icons::TRASH)).clicked() {
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

        if ui.button(format!("{} Texture Input", icons::IMAGE)).clicked() {
            snarl.insert_node(pos, MaterialNode::ImageInput {
                file_path: String::new(),
                is_srgb: true,
                cached_image: None,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Color Value", icons::PALETTE)).clicked() {
            snarl.insert_node(pos, MaterialNode::ColorInput { color: [1.0, 1.0, 1.0, 1.0] });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Float Value", icons::SLIDERS)).clicked() {
            snarl.insert_node(pos, MaterialNode::FloatInput { value: 1.0, min: 0.0, max: 1.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        ui.separator();
        if ui.button(format!("{} Height Generator", icons::WAVES)).clicked() {
            snarl.insert_node(pos, MaterialNode::HeightGenerator { contrast: 1.0, brightness: 0.0, invert: false });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Normal Generator", icons::COMPASS)).clicked() {
            snarl.insert_node(pos, MaterialNode::NormalGenerator { scale: 1.0, filter: NormalFilter::Scharr, orientation: NormalOrientation::DirectX });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Ambient Occlusion (AO)", icons::CIRCLE_HALF)).clicked() {
            snarl.insert_node(pos, MaterialNode::AOGenerator { radius: 16, samples: 16, intensity: 1.0, bias: 0.05 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Curvature / Cavity", icons::APERTURE)).clicked() {
            snarl.insert_node(pos, MaterialNode::CurvatureGenerator { radius: 2, intensity: 2.0, mode: CurvatureMode::Full });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Roughness Remap", icons::SLIDERS)).clicked() {
            snarl.insert_node(pos, MaterialNode::RoughnessGenerator { invert: false, contrast: 1.0, min_val: 0.0, max_val: 1.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Metalness Generator", icons::COIN)).clicked() {
            snarl.insert_node(pos, MaterialNode::MetalnessGenerator {
                threshold: 0.5,
                falloff: 0.2,
                detect_metals: true,
                invert: false,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Emissive Generator", icons::LIGHTBULB)).clicked() {
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
        if ui.button(format!("{} Custom Map Generator", icons::FADERS)).clicked() {
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
        if ui.button(format!("{} Hair Strand Generator", icons::WAVES)).clicked() {
            snarl.insert_node(pos, MaterialNode::HairStrandGenerator {
                strand_density: 250.0,
                roughness: 0.35,
                waviness: 0.20,
                wave_frequency: 4.0,
                orientation: crate::image_proc::StrandOrientation::Vertical,
                normal_intensity: 0.65,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Eye Cornea & Iris Parallax", icons::EYE)).clicked() {
            snarl.insert_node(pos, MaterialNode::EyeCorneaGenerator {
                iris_depth: 0.05,
                cornea_ior: 1.376,
                limbal_width: 0.15,
                limbal_darkness: 0.65,
                caustic_intensity: 1.50,
                dome_curvature: 0.85,
            });
            self.needs_rebuild = true;
            ui.close();
        }
        ui.separator();
        if ui.button(format!("{} Normal Blend (RNM)", icons::GIT_MERGE)).clicked() {
            snarl.insert_node(pos, MaterialNode::NormalBlend { detail_scale: 1.0, detail_tile: 10.0 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Channel Packer (RGBA)", icons::PACKAGE)).clicked() {
            snarl.insert_node(pos, MaterialNode::ChannelPacker { default_r: 128, default_g: 0, default_b: 255, default_a: 255 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Channel Splitter", icons::SCISSORS)).clicked() {
            snarl.insert_node(pos, MaterialNode::ChannelSplitter);
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Color Blend", icons::PAINT_BUCKET)).clicked() {
            snarl.insert_node(pos, MaterialNode::ColorBlend { mode: BlendMode::Mix, factor: 0.5 });
            self.needs_rebuild = true;
            ui.close();
        }
        if ui.button(format!("{} Procedural Noise", icons::WAVEFORM)).clicked() {
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
        self.undo_snapshots.push(snarl.clone());
        snarl.connect(from.id, to.id);
        self.needs_rebuild = true;
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<MaterialNode>) {
        self.undo_snapshots.push(snarl.clone());
        snarl.disconnect(from.id, to.id);
        self.needs_rebuild = true;
    }

    fn drop_outputs(&mut self, pin: &OutPin, snarl: &mut Snarl<MaterialNode>) {
        self.undo_snapshots.push(snarl.clone());
        snarl.drop_outputs(pin.id);
        self.needs_rebuild = true;
    }

    fn drop_inputs(&mut self, pin: &InPin, snarl: &mut Snarl<MaterialNode>) {
        self.undo_snapshots.push(snarl.clone());
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
        ui.add(egui::Label::new(
            egui::RichText::new(name)
                .size(11.0)
                .color(Color32::from_rgb(210, 214, 222)),
        ));
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
        ui.add(egui::Label::new(
            egui::RichText::new(name)
                .size(11.0)
                .color(Color32::from_rgb(210, 214, 222)),
        ));
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

        // Compact Blender spacing & slider width
        ui.spacing_mut().slider_width = 80.0;
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 2.5);
        ui.spacing_mut().button_padding = egui::vec2(5.0, 1.5);

        // Render live node thumbnail if available
        if !matches!(node, MaterialNode::RayMaterialOutput { .. }) {
            if let Some(tex) = self.node_thumbnails.get(&node_id) {
                ui.horizontal(|ui| {
                    ui.image((tex.id(), egui::vec2(36.0, 36.0)));
                    ui.label(egui::RichText::new("Preview").weak().small());
                });
                ui.add_space(1.0);
            }
        }

        match node {
            MaterialNode::ImageInput {
                file_path,
                is_srgb,
                cached_image,
            } => {
                ui.horizontal(|ui| {
                    if ui.button(format!("{} Open...", icons::FOLDER_OPEN)).on_hover_text("Open Image file").clicked() {
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
                        ui.label(egui::RichText::new(short_name).small());
                    }
                });
                changed |= ui.checkbox(is_srgb, "sRGB").changed();
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
                changed |= ui.checkbox(invert, "Invert").changed();
            }
            MaterialNode::NormalGenerator {
                scale,
                filter,
                orientation,
            } => {
                changed |= ui.add(Slider::new(scale, 0.05..=10.0).text("Scale"))
                    .on_hover_text("Perceived 3D depth and sharpness of normal map surface relief")
                    .changed();
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    egui::ComboBox::from_id_salt((node_id, "filt"))
                        .selected_text(match filter {
                            NormalFilter::Sobel => "Sobel",
                            NormalFilter::Scharr => "Scharr",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(filter, NormalFilter::Sobel, "Sobel").changed();
                            changed |= ui.selectable_value(filter, NormalFilter::Scharr, "Scharr").changed();
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    egui::ComboBox::from_id_salt((node_id, "orient"))
                        .selected_text(match orientation {
                            NormalOrientation::DirectX => "DirectX",
                            NormalOrientation::OpenGL => "OpenGL",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(orientation, NormalOrientation::DirectX, "DirectX (Y- Down)")
                                .on_hover_text("Standard for MMD, Ray-MMD, and DirectX games")
                                .changed();
                            changed |= ui.selectable_value(orientation, NormalOrientation::OpenGL, "OpenGL (Y+ Up)")
                                .on_hover_text("Standard for Blender and OpenGL renderers")
                                .changed();
                        });
                });
            }
            MaterialNode::AOGenerator {
                radius,
                samples,
                intensity,
                bias,
            } => {
                changed |= ui.add(Slider::new(radius, 2..=64).text("Radius")).changed();
                changed |= ui.add(Slider::new(samples, 4..=32).text("Rays")).changed();
                changed |= ui.add(Slider::new(intensity, 0.0..=5.0).text("Intensity")).changed();
                changed |= ui.add(Slider::new(bias, -0.2..=0.5).text("Bias")).changed();
            }
            MaterialNode::CurvatureGenerator {
                radius,
                intensity,
                mode,
            } => {
                changed |= ui.add(Slider::new(radius, 1..=16).text("Radius")).changed();
                changed |= ui.add(Slider::new(intensity, 0.1..=10.0).text("Intensity")).changed();
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    egui::ComboBox::from_id_salt((node_id, "curv_mode"))
                        .selected_text(match mode {
                            CurvatureMode::Full => "Full",
                            CurvatureMode::ConvexOnly => "Convex",
                            CurvatureMode::ConcaveOnly => "Concave",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(mode, CurvatureMode::Full, "Full").changed();
                            changed |= ui.selectable_value(mode, CurvatureMode::ConvexOnly, "Convex").changed();
                            changed |= ui.selectable_value(mode, CurvatureMode::ConcaveOnly, "Concave").changed();
                        });
                });
            }
            MaterialNode::RoughnessGenerator {
                invert,
                contrast,
                min_val,
                max_val,
            } => {
                changed |= ui.checkbox(invert, "Invert")
                    .on_hover_text("Inverts between Glossiness (1 = shiny) and Roughness (0 = shiny)")
                    .changed();
                changed |= ui.add(Slider::new(contrast, 0.1..=5.0).text("Contrast")).changed();
                changed |= ui.add(Slider::new(min_val, 0.0..=1.0).text("Min")).changed();
                changed |= ui.add(Slider::new(max_val, 0.0..=1.0).text("Max")).changed();
            }
            MaterialNode::NormalBlend {
                detail_scale,
                detail_tile,
            } => {
                changed |= ui.add(Slider::new(detail_scale, 0.0..=5.0).text("Scale")).changed();
                changed |= ui.add(Slider::new(detail_tile, 1.0..=50.0).text("Tile")).changed();
            }
            MaterialNode::ChannelPacker {
                default_r,
                default_g,
                default_b,
                default_a,
            } => {
                ui.label(egui::RichText::new("Defaults:").small());
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(2.0, 0.0);
                    ui.label("R");
                    changed |= ui.add(DragValue::new(default_r).range(0..=255)).changed();
                    ui.label("G");
                    changed |= ui.add(DragValue::new(default_g).range(0..=255)).changed();
                    ui.label("B");
                    changed |= ui.add(DragValue::new(default_b).range(0..=255)).changed();
                    ui.label("A");
                    changed |= ui.add(DragValue::new(default_a).range(0..=255)).changed();
                });
            }
            MaterialNode::ChannelSplitter => {
                ui.label(egui::RichText::new("Splits RGBA channels").weak().small());
            }
            MaterialNode::ColorBlend { mode, factor } => {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    egui::ComboBox::from_id_salt((node_id, "blend_mode"))
                        .selected_text(match mode {
                            BlendMode::Mix => "Mix",
                            BlendMode::Multiply => "Multiply",
                            BlendMode::Screen => "Screen",
                            BlendMode::Overlay => "Overlay",
                            BlendMode::Add => "Add",
                            BlendMode::Subtract => "Subtract",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(mode, BlendMode::Mix, "Mix").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Multiply, "Multiply").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Screen, "Screen").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Overlay, "Overlay").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Add, "Add").changed();
                            changed |= ui.selectable_value(mode, BlendMode::Subtract, "Subtract").changed();
                        });
                });
                changed |= ui.add(Slider::new(factor, 0.0..=1.0).text("Factor")).changed();
            }
            MaterialNode::ProceduralNoise {
                noise_type,
                scale,
                octaves,
                lacunarity,
                gain,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_salt((node_id, "noise_type_sel"))
                        .selected_text(match noise_type {
                            crate::image_proc::NoiseType::Perlin => "Perlin",
                            crate::image_proc::NoiseType::Voronoi => "Voronoi",
                            crate::image_proc::NoiseType::WhiteNoise => "White",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::Perlin, "Perlin fBm").changed();
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::Voronoi, "Voronoi Cellular").changed();
                            changed |= ui.selectable_value(noise_type, crate::image_proc::NoiseType::WhiteNoise, "White Noise").changed();
                        });
                });
                changed |= ui.add(Slider::new(scale, 0.1..=50.0).text("Scale")).changed();
                if *noise_type == crate::image_proc::NoiseType::Perlin {
                    changed |= ui.add(Slider::new(octaves, 1..=8).text("Octaves")).changed();
                    changed |= ui.add(Slider::new(lacunarity, 1.0..=4.0).text("Lacunarity")).changed();
                    changed |= ui.add(Slider::new(gain, 0.0..=1.0).text("Gain")).changed();
                }
            }
            MaterialNode::MetalnessGenerator {
                threshold,
                falloff,
                detect_metals,
                invert,
            } => {
                changed |= ui.checkbox(detect_metals, "Smart Detect").on_hover_text("Smart Metal Detection (Gold, Copper, Silver)").changed();
                if *detect_metals {
                    changed |= ui.add(Slider::new(threshold, 0.1..=0.9).text("Min Bright")).changed();
                } else {
                    changed |= ui.add(Slider::new(threshold, 0.0..=1.0).text("Threshold")).changed();
                    changed |= ui.add(Slider::new(falloff, 0.01..=0.8).text("Falloff")).changed();
                }
                changed |= ui.checkbox(invert, "Invert").changed();
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
                changed |= ui.add(Slider::new(min_lum, 0.0..=1.0).text("Min Lum")).changed();
                changed |= ui.add(Slider::new(max_lum, 0.0..=1.0).text("Max Lum")).changed();
                changed |= ui.checkbox(use_hue_filter, "Color Keying").changed();
                if *use_hue_filter {
                    changed |= ui.add(Slider::new(target_hue, 0.0..=360.0).text("Hue")).changed();
                    changed |= ui.add(Slider::new(hue_tolerance, 5.0..=120.0).text("Tol")).changed();
                }
                ui.horizontal(|ui| {
                    ui.label("Tint:");
                    changed |= ui.color_edit_button_rgb(tint_color).changed();
                });
                changed |= ui.add(Slider::new(intensity, 0.0..=20.0).text("Boost")).changed();
                changed |= ui.checkbox(invert, "Invert").changed();
            }
            MaterialNode::CustomMapGenerator {
                model,
                param_a,
                param_b_color,
                invert_a,
                aniso_radial,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Model:");
                    egui::ComboBox::from_id_salt((node_id, "cust_model"))
                        .selected_text(model.short_name())
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

                changed |= ui.add(Slider::new(param_a, 0.0..=1.0).text(model.custom_a_name())).changed();
                changed |= ui.checkbox(invert_a, "Invert A").changed();

                if *model != crate::graph::node::ShadingModel::ClearCoat {
                    if *model == crate::graph::node::ShadingModel::Anisotropy {
                        changed |= ui.checkbox(aniso_radial, "Radial Tangent").changed();
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Tint:");
                            changed |= ui.color_edit_button_rgb(param_b_color).changed();
                        });
                    }
                }
            }
            MaterialNode::HairStrandGenerator {
                strand_density,
                roughness,
                waviness,
                wave_frequency,
                orientation,
                normal_intensity,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Orient:");
                    egui::ComboBox::from_id_salt((node_id, "strand_ori"))
                        .selected_text(match orientation {
                            crate::image_proc::StrandOrientation::Vertical => "Vertical (Y)",
                            crate::image_proc::StrandOrientation::Horizontal => "Horizontal (X)",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(orientation, crate::image_proc::StrandOrientation::Vertical, "Vertical (Y)").changed();
                            changed |= ui.selectable_value(orientation, crate::image_proc::StrandOrientation::Horizontal, "Horizontal (X)").changed();
                        });
                });
                changed |= ui.add(Slider::new(strand_density, 20.0..=800.0).text("Density")).changed();
                changed |= ui.add(Slider::new(roughness, 0.0..=1.0).text("Roughness")).changed();
                changed |= ui.add(Slider::new(waviness, 0.0..=1.0).text("Waviness")).changed();
                changed |= ui.add(Slider::new(wave_frequency, 0.5..=20.0).text("Wave Freq")).changed();
                changed |= ui.add(Slider::new(normal_intensity, 0.0..=2.0).text("Normal Int")).changed();
            }
            MaterialNode::EyeCorneaGenerator {
                iris_depth,
                cornea_ior,
                limbal_width,
                limbal_darkness,
                caustic_intensity,
                dome_curvature,
            } => {
                changed |= ui.add(Slider::new(dome_curvature, 0.1..=1.0).text("Dome Curv")).changed();
                changed |= ui.add(Slider::new(iris_depth, 0.005..=0.20).text("Iris Depth")).changed();
                changed |= ui.add(Slider::new(cornea_ior, 1.0..=1.8).text("Cornea IOR")).changed();
                changed |= ui.add(Slider::new(limbal_width, 0.02..=0.40).text("Limbal Width")).changed();
                changed |= ui.add(Slider::new(limbal_darkness, 0.0..=1.0).text("Limbal Dark")).changed();
                changed |= ui.add(Slider::new(caustic_intensity, 0.0..=3.0).text("Caustic Int")).changed();
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
                normal_sub_scale,
                normal_sub_loop,
                procedural_hair_enable,
                procedural_hair_scale,
                procedural_hair_intensity,
                eye_parallax_enable,
                eye_iris_depth,
                eye_cornea_ior,
                convex_normal_enable,
                ..
            } => {
                ui.vertical(|ui| {
                    ui.set_min_width(200.0);

                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        changed |= ui.add(egui::TextEdit::singleline(material_name).desired_width(110.0)).changed();
                    });

                    ui.horizontal(|ui| {
                        ui.label("Model:");
                        egui::ComboBox::from_id_salt((node_id, "out_model"))
                            .selected_text(shading_model.short_name())
                            .width(110.0)
                            .show_ui(ui, |ui| {
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Default, "0: Default PBR").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Skin, "1: Skin (SSS)").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Anisotropy, "3: Anisotropy").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Glass, "4: Glass").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Cloth, "5: Cloth").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::ClearCoat, "6: Clear Coat").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::Subsurface, "7: Subsurface").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::CelShading, "8: Cel Shading").changed();
                                changed |= ui.selectable_value(shading_model, crate::graph::node::ShadingModel::ToonShading, "9: Toon Shading").changed();
                            });
                    });

                    changed |= ui.checkbox(is_roughness_mode, "Roughness Mode")
                        .on_hover_text("Auto-invert for Ray-MMD Smoothness")
                        .changed();

                    ui.collapsing("Emission & Blink", |ui| {
                        changed |= ui.add(Slider::new(emissive_intensity, 0.0..=50.0).text("Intensity")).changed();
                        ui.horizontal(|ui| {
                            ui.label("Blink:");
                            egui::ComboBox::from_id_salt((node_id, "blink_m"))
                                .selected_text(match *emissive_blink_mode {
                                    0 => "None",
                                    1 => "Freq",
                                    _ => "Morph",
                                })
                                .show_ui(ui, |ui| {
                                    changed |= ui.selectable_value(emissive_blink_mode, 0, "None").changed();
                                    changed |= ui.selectable_value(emissive_blink_mode, 1, "Constant Frequency").changed();
                                    changed |= ui.selectable_value(emissive_blink_mode, 2, "Morph Controller").changed();
                                });
                        });
                        if *emissive_blink_mode == 1 {
                            ui.horizontal(|ui| {
                                ui.label("Freq:");
                                changed |= ui.add(DragValue::new(&mut emissive_blink_freq[0]).speed(0.1)).changed();
                                changed |= ui.add(DragValue::new(&mut emissive_blink_freq[1]).speed(0.1)).changed();
                                changed |= ui.add(DragValue::new(&mut emissive_blink_freq[2]).speed(0.1)).changed();
                            });
                        }
                    });

                    ui.collapsing("Extended FX", |ui| {
                        changed |= ui.checkbox(hex_tiling_enable, "Hex-Tiling").on_hover_text("Mikkelsen 2022 procedural repetition suppression").changed();
                        changed |= ui.checkbox(hashed_alpha_enable, "Hashed Alpha").on_hover_text("Subpixel stochastic alpha cutout").changed();
                        changed |= ui.checkbox(detail_map_enable, "Detail Normal").on_hover_text("Micro-surface normal detail overlay").changed();
                        ui.separator();
                        ui.label("Sub-Normal Map:");
                        changed |= ui.add(Slider::new(normal_sub_scale, 0.0..=3.0).text("Sub Scale")).changed();
                        changed |= ui.add(Slider::new(normal_sub_loop, 1.0..=50.0).text("Sub Loop")).changed();
                    });

                    ui.collapsing("Hair & Eye Special FX", |ui| {
                        changed |= ui.checkbox(procedural_hair_enable, "Procedural Hair")
                            .on_hover_text("Ray-MMD anisotropic strand micro-groove perturbation")
                            .changed();
                        if *procedural_hair_enable {
                            changed |= ui.add(Slider::new(procedural_hair_scale, 0.01..=2.0).text("Hair Scale")).changed();
                            changed |= ui.add(Slider::new(procedural_hair_intensity, 0.0..=2.0).text("Hair Intensity")).changed();
                        }
                        ui.separator();
                        changed |= ui.checkbox(convex_normal_enable, "Convex Eye Normal")
                            .on_hover_text("Invert concave anime eye curvature into convex cornea dome")
                            .changed();
                        changed |= ui.checkbox(eye_parallax_enable, "Eye Iris Parallax")
                            .on_hover_text("Snell's Law cornea refraction into anterior chamber")
                            .changed();
                        if *eye_parallax_enable {
                            changed |= ui.add(Slider::new(eye_iris_depth, 0.005..=0.25).text("Iris Depth")).changed();
                            changed |= ui.add(Slider::new(eye_cornea_ior, 1.0..=2.0).text("Cornea IOR")).changed();
                        }
                    });
                });
            }
        }

        if changed {
            self.needs_rebuild = true;
        }
    }
}
