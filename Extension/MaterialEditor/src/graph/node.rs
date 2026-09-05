use crate::image_proc::{CurvatureMode, NoiseType, NormalFilter, NormalOrientation, StrandOrientation, U8Image};
use egui::Color32;
use serde::{Deserialize, Serialize};

/// Supported pin data types in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinType {
    Rgba,
    Grayscale,
    Float,
    Vector,
}

/// Node variants supported in the material graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialNode {
    /// Generates multi-octave procedural noise (Perlin fBm, Voronoi cellular, White noise).
    ProceduralNoise {
        noise_type: NoiseType,
        scale: f32,
        octaves: usize,
        lacunarity: f32,
        gain: f32,
    },
    /// Loads an image file from disk.
    ImageInput {
        file_path: String,
        is_srgb: bool,
        #[serde(skip)]
        cached_image: Option<U8Image>,
    },
    /// Constant RGBA color input.
    ColorInput {
        color: [f32; 4],
    },
    /// Constant scalar float input.
    FloatInput {
        value: f32,
        min: f32,
        max: f32,
    },
    /// ShaderMap: Generates height / displacement from diffuse luminance.
    HeightGenerator {
        contrast: f32,
        brightness: f32,
        invert: bool,
    },
    /// ShaderMap: Generates tangent-space normal map from height or diffuse.
    NormalGenerator {
        scale: f32,
        filter: NormalFilter,
        orientation: NormalOrientation,
    },
    /// ShaderMap: Horizon-marching ambient occlusion on heightfield.
    AOGenerator {
        radius: usize,
        samples: usize,
        intensity: f32,
        bias: f32,
    },
    /// ShaderMap: Curvature / cavity / edge wear from height.
    CurvatureGenerator {
        radius: usize,
        intensity: f32,
        mode: CurvatureMode,
    },
    /// ShaderMap: Roughness / smoothness curve remapping.
    RoughnessGenerator {
        invert: bool,
        contrast: f32,
        min_val: f32,
        max_val: f32,
    },
    /// Reoriented Normal Mapping (RNM) blending of base normal with detail normal.
    NormalBlend {
        detail_scale: f32,
        detail_tile: f32,
    },
    /// Packs individual channels into a single RGBA map (e.g. RMA: Roughness, Metalness, AO).
    ChannelPacker {
        default_r: u8,
        default_g: u8,
        default_b: u8,
        default_a: u8,
    },
    /// Splits an RGBA texture into individual R, G, B, A grayscale maps.
    ChannelSplitter,
    /// Color blend operations.
    ColorBlend {
        mode: BlendMode,
        factor: f32,
    },
    /// ShaderMap: Generates metallic mask from albedo colors, saturation, and threshold.
    MetalnessGenerator {
        threshold: f32,
        falloff: f32,
        detect_metals: bool,
        invert: bool,
    },
    /// ShaderMap: Generates glowing emissive map from luminance thresholding, hue keying, tint, and boost.
    EmissiveGenerator {
        min_lum: f32,
        max_lum: f32,
        use_hue_filter: bool,
        target_hue: f32,
        hue_tolerance: f32,
        tint_color: [f32; 3],
        intensity: f32,
        invert: bool,
    },
    /// ShaderMap / Ray-MMD: Generates Custom A and Custom B maps for specific Shading Models.
    CustomMapGenerator {
        model: ShadingModel,
        param_a: f32,
        param_b_color: [f32; 3],
        invert_a: bool,
        aniso_radial: bool,
    },
    /// Ray-MMD: Generates anisotropic hair strand normal, silky tangent shift (shift4.png), and strand mask.
    HairStrandGenerator {
        strand_density: f32,
        roughness: f32,
        waviness: f32,
        wave_frequency: f32,
        orientation: StrandOrientation,
        normal_intensity: f32,
    },
    /// Ray-MMD: Generates cornea convex dome normal, iris parallax depth, limbal & caustic, and refracted iris.
    EyeCorneaGenerator {
        iris_depth: f32,
        cornea_ior: f32,
        limbal_width: f32,
        limbal_darkness: f32,
        caustic_intensity: f32,
        dome_curvature: f32,
    },
    /// Master Ray-MMD ReForge Material Output node.
    RayMaterialOutput {
        material_name: String,
        shading_model: ShadingModel,
        albedo_color: [f32; 3],
        albedo_loop: [f32; 2],
        normal_scale: f32,
        normal_loop: f32,
        smoothness_val: f32,
        is_roughness_mode: bool,
        metalness_val: f32,
        specular_color: [f32; 3],
        occlusion_val: f32,
        parallax_scale: f32,
        emissive_color: [f32; 3],
        emissive_intensity: f32,
        emissive_blink_mode: u32,
        emissive_blink_freq: [f32; 3],
        custom_a_val: f32,
        custom_b_color: [f32; 3],
        // Hex Tiling
        hex_tiling_enable: bool,
        hex_tiling_rotation: f32,
        hex_tiling_contrast: f32,
        hex_tiling_sharpness: f32,
        // Hashed Alpha
        hashed_alpha_enable: bool,
        hashed_alpha_scale: f32,
        // Detail Normal
        detail_map_enable: bool,
        detail_normal_scale: f32,
        detail_normal_loop: f32,
        detail_fade_distance: f32,
        // Sub-Normal
        normal_sub_scale: f32,
        normal_sub_loop: f32,
        // Procedural Hair
        procedural_hair_enable: bool,
        procedural_hair_scale: f32,
        procedural_hair_intensity: f32,
        // Eye Parallax & Cornea Dome
        eye_parallax_enable: bool,
        eye_iris_depth: f32,
        eye_cornea_ior: f32,
        convex_normal_enable: bool,
    },
}

/// Ray-MMD Shading Model ID (`CUSTOM_ENABLE` parameter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadingModel {
    Default = 0,
    Skin = 1,
    Anisotropy = 3,
    Glass = 4,
    Cloth = 5,
    ClearCoat = 6,
    Subsurface = 7,
    CelShading = 8,
    ToonShading = 9,
}

impl ShadingModel {
    pub fn id(&self) -> u32 {
        match self {
            Self::Default => 0,
            Self::Skin => 1,
            Self::Anisotropy => 3,
            Self::Glass => 4,
            Self::Cloth => 5,
            Self::ClearCoat => 6,
            Self::Subsurface => 7,
            Self::CelShading => 8,
            Self::ToonShading => 9,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Default => "0: Default (Standard PBR)",
            Self::Skin => "1: PreIntegrated Skin (Curvature + SSS Tint)",
            Self::Anisotropy => "3: Anisotropy (Hair / Brushed Metal)",
            Self::Glass => "4: Glass (Curvature + Transmittance)",
            Self::Cloth => "5: Cloth (Sheen GGX + Fuzz Color)",
            Self::ClearCoat => "6: Clear Coat (Lacquer / Car Paint)",
            Self::Subsurface => "7: Subsurface Profile (Curvature + SSS)",
            Self::CelShading => "8: Cel Shading (Threshold + Shadow Tint)",
            Self::ToonShading => "9: Toon Shading (Hardness + Shadow Tint)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Default => "0: Default",
            Self::Skin => "1: Skin",
            Self::Anisotropy => "3: Aniso",
            Self::Glass => "4: Glass",
            Self::Cloth => "5: Cloth",
            Self::ClearCoat => "6: Clear Coat",
            Self::Subsurface => "7: Subsurface",
            Self::CelShading => "8: Cel Shading",
            Self::ToonShading => "9: Toon Shading",
        }
    }

    pub fn custom_a_name(&self) -> &'static str {
        match self {
            Self::Default => "Custom A (SSS)",
            Self::Skin => "Curvature / SSS",
            Self::Anisotropy => "Aniso Strength",
            Self::Glass => "Absorption",
            Self::Cloth => "Sheen Weight",
            Self::ClearCoat => "Coat Smoothness",
            Self::Subsurface => "SSS Opacity",
            Self::CelShading => "Step Threshold",
            Self::ToonShading => "Ramp Hardness",
        }
    }

    pub fn custom_b_name(&self) -> &'static str {
        match self {
            Self::Default => "Custom B (Tint)",
            Self::Skin => "Transmittance",
            Self::Anisotropy => "Tangent Flow",
            Self::Glass => "Transmittance",
            Self::Cloth => "Fuzz Color",
            Self::ClearCoat => "Coat Normal",
            Self::Subsurface => "Transmittance",
            Self::CelShading => "Shadow Tint",
            Self::ToonShading => "Shadow Tint",
        }
    }
}

/// Blend modes for ColorBlend node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    Mix,
    Multiply,
    Screen,
    Overlay,
    Add,
    Subtract,
}

impl MaterialNode {
    /// Returns the display title for the node.
    pub fn title(&self) -> &'static str {
        match self {
            Self::ProceduralNoise { .. } => "Procedural Noise",
            Self::ImageInput { .. } => "Texture Input",
            Self::ColorInput { .. } => "Color Value",
            Self::FloatInput { .. } => "Float Value",
            Self::HeightGenerator { .. } => "Height Generator",
            Self::NormalGenerator { .. } => "Normal Generator",
            Self::AOGenerator { .. } => "Ambient Occlusion",
            Self::CurvatureGenerator { .. } => "Curvature / Cavity",
            Self::RoughnessGenerator { .. } => "Roughness Remap",
            Self::NormalBlend { .. } => "Normal Blend (RNM)",
            Self::ChannelPacker { .. } => "Channel Packer (RGBA)",
            Self::ChannelSplitter => "Channel Splitter",
            Self::ColorBlend { .. } => "Color Blend",
            Self::MetalnessGenerator { .. } => "Metalness Generator",
            Self::EmissiveGenerator { .. } => "Emissive Generator",
            Self::CustomMapGenerator { .. } => "Custom Map Generator",
            Self::HairStrandGenerator { .. } => "Hair Strand Generator",
            Self::EyeCorneaGenerator { .. } => "Eye Cornea & Iris Parallax",
            Self::RayMaterialOutput { .. } => "Ray-MMD Material Output",
        }
    }

    /// Category header accent color matching Blender Shader Editor conventions.
    pub fn category_color(&self) -> egui::Color32 {
        match self {
            // Blender Texture (Warm Orange)
            Self::ProceduralNoise { .. } | Self::ImageInput { .. } => {
                Color32::from_rgb(186, 92, 38)
            }
            // Blender Input (Slate / Charcoal)
            Self::ColorInput { .. } | Self::FloatInput { .. } => {
                Color32::from_rgb(80, 88, 98)
            }
            // Blender Vector / Normal (Purple)
            Self::NormalGenerator { .. } | Self::NormalBlend { .. } => {
                Color32::from_rgb(114, 72, 172)
            }
            // Blender Color / Converter (Deep Slate Blue)
            Self::ChannelPacker { .. } | Self::ChannelSplitter | Self::ColorBlend { .. } => {
                Color32::from_rgb(44, 102, 168)
            }
            // Blender Shader / Converter Filters (Teal)
            Self::HeightGenerator { .. }
            | Self::AOGenerator { .. }
            | Self::CurvatureGenerator { .. }
            | Self::RoughnessGenerator { .. }
            | Self::MetalnessGenerator { .. }
            | Self::EmissiveGenerator { .. }
            | Self::CustomMapGenerator { .. }
            | Self::HairStrandGenerator { .. }
            | Self::EyeCorneaGenerator { .. } => {
                Color32::from_rgb(40, 126, 124)
            }
            // Blender Material Output (Emerald Green)
            Self::RayMaterialOutput { .. } => {
                Color32::from_rgb(42, 132, 74)
            }
        }
    }

    /// Number of input pins.
    pub fn input_count(&self) -> usize {
        match self {
            Self::ProceduralNoise { .. } => 0,
            Self::ImageInput { .. } => 0,
            Self::ColorInput { .. } => 0,
            Self::FloatInput { .. } => 0,
            Self::HeightGenerator { .. } => 1,
            Self::NormalGenerator { .. } => 1,
            Self::AOGenerator { .. } => 1,
            Self::CurvatureGenerator { .. } => 1,
            Self::RoughnessGenerator { .. } => 1,
            Self::NormalBlend { .. } => 2,
            Self::ChannelPacker { .. } => 4,
            Self::ChannelSplitter => 1,
            Self::ColorBlend { .. } => 2,
            Self::MetalnessGenerator { .. } => 1,
            Self::EmissiveGenerator { .. } => 2,
            Self::CustomMapGenerator { .. } => 2,
            Self::HairStrandGenerator { .. } => 0,
            Self::EyeCorneaGenerator { .. } => 1,
            Self::RayMaterialOutput { .. } => 14,
        }
    }

    /// Number of output pins.
    pub fn output_count(&self) -> usize {
        match self {
            Self::ProceduralNoise { .. } => 1,
            Self::ImageInput { .. } => 1,
            Self::ColorInput { .. } => 1,
            Self::FloatInput { .. } => 1,
            Self::HeightGenerator { .. } => 1,
            Self::NormalGenerator { .. } => 1,
            Self::AOGenerator { .. } => 1,
            Self::CurvatureGenerator { .. } => 1,
            Self::RoughnessGenerator { .. } => 1,
            Self::NormalBlend { .. } => 1,
            Self::ChannelPacker { .. } => 1,
            Self::ChannelSplitter => 4,
            Self::ColorBlend { .. } => 1,
            Self::MetalnessGenerator { .. } => 1,
            Self::EmissiveGenerator { .. } => 1,
            Self::CustomMapGenerator { .. } => 2,
            Self::HairStrandGenerator { .. } => 3,
            Self::EyeCorneaGenerator { .. } => 4,
            Self::RayMaterialOutput { .. } => 0,
        }
    }

    /// Name and type of input pin by index.
    pub fn input_info(&self, index: usize) -> (&'static str, PinType) {
        match self {
            Self::HeightGenerator { .. } => ("Diffuse / RGB", PinType::Rgba),
            Self::NormalGenerator { .. } => ("Height / Diffuse", PinType::Grayscale),
            Self::AOGenerator { .. } => ("Height", PinType::Grayscale),
            Self::CurvatureGenerator { .. } => ("Height", PinType::Grayscale),
            Self::RoughnessGenerator { .. } => ("Input", PinType::Grayscale),
            Self::NormalBlend { .. } => match index {
                0 => ("Base Normal", PinType::Vector),
                _ => ("Detail Normal", PinType::Vector),
            },
            Self::ChannelPacker { .. } => match index {
                0 => ("R", PinType::Grayscale),
                1 => ("G", PinType::Grayscale),
                2 => ("B", PinType::Grayscale),
                _ => ("A", PinType::Grayscale),
            },
            Self::ChannelSplitter => ("RGBA In", PinType::Rgba),
            Self::ColorBlend { .. } => match index {
                0 => ("Base", PinType::Rgba),
                _ => ("Blend", PinType::Rgba),
            },
            Self::MetalnessGenerator { .. } => ("Diffuse / Albedo", PinType::Rgba),
            Self::EmissiveGenerator { .. } => match index {
                0 => ("Base / Diffuse", PinType::Rgba),
                _ => ("Mask (Optional)", PinType::Grayscale),
            },
            Self::CustomMapGenerator { .. } => match index {
                0 => ("Guide / Height (Optional)", PinType::Grayscale),
                _ => ("Guide / Color (Optional)", PinType::Rgba),
            },
            Self::HairStrandGenerator { .. } => ("", PinType::Rgba),
            Self::EyeCorneaGenerator { .. } => ("Iris Color (Optional)", PinType::Rgba),
            Self::RayMaterialOutput { shading_model, .. } => match index {
                0 => ("Albedo", PinType::Rgba),
                1 => ("Albedo Sub", PinType::Rgba),
                2 => ("Alpha", PinType::Grayscale),
                3 => ("Normal", PinType::Vector),
                4 => ("Roughness", PinType::Grayscale),
                5 => ("Metalness", PinType::Grayscale),
                6 => ("Specular", PinType::Rgba),
                7 => ("AO", PinType::Grayscale),
                8 => ("Height", PinType::Grayscale),
                9 => ("Emissive", PinType::Rgba),
                10 => (shading_model.custom_a_name(), PinType::Grayscale),
                11 => (shading_model.custom_b_name(), PinType::Rgba),
                12 => ("Detail Normal", PinType::Vector),
                _ => ("Normal Sub", PinType::Vector),
            },
            _ => ("", PinType::Rgba),
        }
    }

    /// Name and type of output pin by index.
    pub fn output_info(&self, index: usize) -> (&'static str, PinType) {
        match self {
            Self::ProceduralNoise { .. } => ("Noise (Grayscale)", PinType::Grayscale),
            Self::ImageInput { .. } => ("RGBA", PinType::Rgba),
            Self::ColorInput { .. } => ("RGBA", PinType::Rgba),
            Self::FloatInput { .. } => ("Float", PinType::Float),
            Self::HeightGenerator { .. } => ("Height", PinType::Grayscale),
            Self::NormalGenerator { .. } => ("Normal", PinType::Vector),
            Self::AOGenerator { .. } => ("AO", PinType::Grayscale),
            Self::CurvatureGenerator { .. } => ("Curvature", PinType::Grayscale),
            Self::RoughnessGenerator { .. } => ("Roughness", PinType::Grayscale),
            Self::NormalBlend { .. } => ("Blended Normal", PinType::Vector),
            Self::ChannelPacker { .. } => ("Packed RGBA", PinType::Rgba),
            Self::ChannelSplitter => match index {
                0 => ("R", PinType::Grayscale),
                1 => ("G", PinType::Grayscale),
                2 => ("B", PinType::Grayscale),
                _ => ("A", PinType::Grayscale),
            },
            Self::ColorBlend { .. } => ("Result", PinType::Rgba),
            Self::MetalnessGenerator { .. } => ("Metalness", PinType::Grayscale),
            Self::EmissiveGenerator { .. } => ("Emissive", PinType::Rgba),
            Self::CustomMapGenerator { model, .. } => match index {
                0 => (model.custom_a_name(), PinType::Grayscale),
                _ => (model.custom_b_name(), PinType::Rgba),
            },
            Self::HairStrandGenerator { .. } => match index {
                0 => ("Normal (Strands)", PinType::Vector),
                1 => ("Tangent Shift (Silky)", PinType::Grayscale),
                _ => ("Strand Mask", PinType::Grayscale),
            },
            Self::EyeCorneaGenerator { .. } => match index {
                0 => ("Cornea Dome Normal", PinType::Vector),
                1 => ("Iris Parallax Map", PinType::Grayscale),
                2 => ("Limbal & Caustic Map", PinType::Rgba),
                _ => ("Refracted Iris", PinType::Rgba),
            },
            Self::RayMaterialOutput { .. } => ("", PinType::Rgba),
        }
    }
}
