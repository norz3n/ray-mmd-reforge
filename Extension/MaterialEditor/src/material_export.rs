//! Ray-MMD ReForge material export generator.
//!
//! Generates `material.fx` files adhering strictly to `material_2.0.fx` and `material_common_2.0.fxsub`
//! format, with full support for PBR textures, hex-tiling, detail maps, and custom channels.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::{Context, Result};
use crate::image_proc::U8Image;

/// Material configuration matching ray-mmd 2.0 specifications.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RayMaterialConfig {
    pub name: String,

    // Albedo
    pub albedo_enabled: bool,
    pub albedo_file: String,
    pub albedo_color: [f32; 3],
    pub albedo_loop: [f32; 2],

    // Albedo Sub
    pub albedo_sub_enabled: bool,
    pub albedo_sub_file: String,
    pub albedo_sub_color: [f32; 3],
    pub albedo_sub_loop: [f32; 2],

    // Alpha
    pub alpha_enabled: bool,
    pub alpha_file: String,
    pub alpha_val: f32,
    pub alpha_swizzle: u32, // 0: R, 1: G, 2: B, 3: A

    // Normal
    pub normal_enabled: bool,
    pub normal_file: String,
    pub normal_scale: f32,
    pub normal_loop: f32,

    // Smoothness / Roughness
    pub smoothness_enabled: bool,
    pub smoothness_file: String,
    pub smoothness_val: f32,
    pub smoothness_swizzle: u32,
    pub is_roughness_mode: bool, // true: invert when interpreting

    // Metalness
    pub metalness_enabled: bool,
    pub metalness_file: String,
    pub metalness_val: f32,
    pub metalness_swizzle: u32,

    // Specular
    pub specular_enabled: bool,
    pub specular_file: String,
    pub specular_color: [f32; 3],

    // Occlusion (AO)
    pub occlusion_enabled: bool,
    pub occlusion_file: String,
    pub occlusion_val: f32,
    pub occlusion_swizzle: u32,

    // Parallax / Height
    pub parallax_enabled: bool,
    pub parallax_file: String,
    pub parallax_scale: f32,

    // Emissive
    pub emissive_enabled: bool,
    pub emissive_file: String,
    pub emissive_color: [f32; 3],
    pub emissive_intensity: f32,
    pub emissive_blink: [f32; 3],
    pub emissive_blink_mode: u32,

    // Custom channels
    pub custom_enabled: bool,
    pub shading_model_id: u32,
    pub custom_a_enabled: bool,
    pub custom_a_file: String,
    pub custom_a_val: f32,
    pub custom_b_enabled: bool,
    pub custom_b_file: String,
    pub custom_b_color: [f32; 3],

    // Hex-Tiling (Mikkelsen 2022)
    pub hex_tiling_enable: bool,
    pub hex_tiling_rotation: f32,
    pub hex_tiling_contrast: f32,
    pub hex_tiling_sharpness: f32,
    pub hex_tiling_distance_lod: bool,
    pub hex_tiling_lod_start: f32,
    pub hex_tiling_lod_end: f32,

    // Hashed Alpha Test
    pub hashed_alpha_enable: bool,
    pub hashed_alpha_scale: f32,

    // Detail Normal Map
    pub detail_map_enable: bool,
    pub detail_normal_file: String,
    pub detail_normal_scale: f32,
    pub detail_normal_loop: f32,
    pub detail_fade_distance: f32,
}

impl Default for RayMaterialConfig {
    fn default() -> Self {
        Self {
            name: "reforge_material".to_string(),

            albedo_enabled: true,
            albedo_file: "albedo.png".to_string(),
            albedo_color: [1.0, 1.0, 1.0],
            albedo_loop: [1.0, 1.0],

            albedo_sub_enabled: false,
            albedo_sub_file: "albedo_sub.png".to_string(),
            albedo_sub_color: [1.0, 1.0, 1.0],
            albedo_sub_loop: [1.0, 1.0],

            alpha_enabled: false,
            alpha_file: "alpha.png".to_string(),
            alpha_val: 1.0,
            alpha_swizzle: 3,

            normal_enabled: true,
            normal_file: "normal.png".to_string(),
            normal_scale: 1.0,
            normal_loop: 1.0,

            smoothness_enabled: true,
            smoothness_file: "smoothness.png".to_string(),
            smoothness_val: 0.5,
            smoothness_swizzle: 0,
            is_roughness_mode: false,

            metalness_enabled: true,
            metalness_file: "metalness.png".to_string(),
            metalness_val: 0.0,
            metalness_swizzle: 0,

            specular_enabled: false,
            specular_file: "specular.png".to_string(),
            specular_color: [0.5, 0.5, 0.5],

            occlusion_enabled: true,
            occlusion_file: "occlusion.png".to_string(),
            occlusion_val: 1.0,
            occlusion_swizzle: 0,

            parallax_enabled: false,
            parallax_file: "height.png".to_string(),
            parallax_scale: 0.05,

            emissive_enabled: false,
            emissive_file: "emissive.png".to_string(),
            emissive_color: [1.0, 1.0, 1.0],
            emissive_intensity: 1.0,
            emissive_blink: [1.0, 1.0, 1.0],
            emissive_blink_mode: 0,

            custom_enabled: false,
            shading_model_id: 0,
            custom_a_enabled: false,
            custom_a_file: "custom_a.png".to_string(),
            custom_a_val: 0.0,
            custom_b_enabled: false,
            custom_b_file: "custom_b.png".to_string(),
            custom_b_color: [0.0, 0.0, 0.0],

            hex_tiling_enable: false,
            hex_tiling_rotation: 1.0,
            hex_tiling_contrast: 0.6,
            hex_tiling_sharpness: 7.0,
            hex_tiling_distance_lod: false,
            hex_tiling_lod_start: 15.0,
            hex_tiling_lod_end: 35.0,

            hashed_alpha_enable: false,
            hashed_alpha_scale: 1.0,

            detail_map_enable: false,
            detail_normal_file: "detail_normal.png".to_string(),
            detail_normal_scale: 1.0,
            detail_normal_loop: 20.0,
            detail_fade_distance: 15.0,
        }
    }
}

impl RayMaterialConfig {
    /// Generates the complete HLSL code for `material.fx`.
    pub fn generate_fx_code(&self) -> String {
        let mut code = String::with_capacity(4096);

        code.push_str("// =============================================================================\n");
        code.push_str(&format!("// Ray-MMD ReForge Generated Material: {}\n", self.name));
        code.push_str("// Generated by ReForge Material Editor (Rust)\n");
        code.push_str("// =============================================================================\n\n");

        // Albedo
        let albedo_from = if self.albedo_enabled { 3 } else { 0 };
        code.push_str(&format!("#define ALBEDO_MAP_FROM {}\n", albedo_from));
        code.push_str("#define ALBEDO_MAP_UV_FLIP 0\n");
        code.push_str("#define ALBEDO_MAP_APPLY_SCALE 0\n");
        code.push_str("#define ALBEDO_MAP_APPLY_DIFFUSE 1\n");
        code.push_str("#define ALBEDO_MAP_APPLY_MORPH_COLOR 0\n");
        code.push_str(&format!("#define ALBEDO_MAP_FILE \"{}\"\n\n", self.albedo_file));
        code.push_str(&format!(
            "const float3 albedo = float3({:.4}, {:.4}, {:.4});\n",
            self.albedo_color[0], self.albedo_color[1], self.albedo_color[2]
        ));
        code.push_str(&format!(
            "const float2 albedoMapLoopNum = float2({:.4}, {:.4});\n\n",
            self.albedo_loop[0], self.albedo_loop[1]
        ));

        // Albedo Sub
        let albedo_sub_en = if self.albedo_sub_enabled { 1 } else { 0 };
        let albedo_sub_from = if self.albedo_sub_enabled { 3 } else { 0 };
        code.push_str(&format!("#define ALBEDO_SUB_ENABLE {}\n", albedo_sub_en));
        code.push_str(&format!("#define ALBEDO_SUB_MAP_FROM {}\n", albedo_sub_from));
        code.push_str("#define ALBEDO_SUB_MAP_UV_FLIP 0\n");
        code.push_str("#define ALBEDO_SUB_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define ALBEDO_SUB_MAP_FILE \"{}\"\n\n", self.albedo_sub_file));
        code.push_str(&format!(
            "const float3 albedoSub = float3({:.4}, {:.4}, {:.4});\n",
            self.albedo_sub_color[0], self.albedo_sub_color[1], self.albedo_sub_color[2]
        ));
        code.push_str(&format!(
            "const float2 albedoSubMapLoopNum = float2({:.4}, {:.4});\n\n",
            self.albedo_sub_loop[0], self.albedo_sub_loop[1]
        ));

        // Alpha
        let alpha_from = if self.alpha_enabled { 3 } else { 0 };
        code.push_str(&format!("#define ALPHA_MAP_FROM {}\n", alpha_from));
        code.push_str("#define ALPHA_MAP_UV_FLIP 0\n");
        code.push_str(&format!("#define ALPHA_MAP_SWIZZLE {}\n", self.alpha_swizzle));
        code.push_str(&format!("#define ALPHA_MAP_FILE \"{}\"\n\n", self.alpha_file));
        code.push_str(&format!("const float alpha = {:.4};\n", self.alpha_val));
        code.push_str("const float alphaMapLoopNum = 1.0;\n\n");

        // Normal Map
        let normal_from = if self.normal_enabled { 3 } else { 0 };
        code.push_str(&format!("#define NORMAL_MAP_FROM {}\n", normal_from));
        code.push_str("#define NORMAL_MAP_TYPE 0\n");
        code.push_str("#define NORMAL_MAP_UV_FLIP 0\n");
        code.push_str(&format!("#define NORMAL_MAP_FILE \"{}\"\n\n", self.normal_file));
        code.push_str(&format!("const float normalMapScale = {:.4};\n", self.normal_scale));
        code.push_str(&format!("const float normalMapLoopNum = {:.4};\n\n", self.normal_loop));

        // Smoothness
        let smoothness_from = if self.smoothness_enabled { 3 } else { 0 };
        code.push_str(&format!("#define SMOOTHNESS_MAP_FROM {}\n", smoothness_from));
        code.push_str("#define SMOOTHNESS_MAP_TYPE 0\n");
        code.push_str("#define SMOOTHNESS_MAP_UV_FLIP 0\n");
        code.push_str(&format!("#define SMOOTHNESS_MAP_SWIZZLE {}\n", self.smoothness_swizzle));
        code.push_str("#define SMOOTHNESS_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define SMOOTHNESS_MAP_FILE \"{}\"\n\n", self.smoothness_file));
        let final_smoothness = if self.is_roughness_mode {
            1.0 - self.smoothness_val
        } else {
            self.smoothness_val
        };
        code.push_str(&format!("const float smoothness = {:.4};\n", final_smoothness));
        code.push_str("const float smoothnessMapLoopNum = 1.0;\n\n");

        // Metalness
        let metalness_from = if self.metalness_enabled { 3 } else { 0 };
        code.push_str(&format!("#define METALNESS_MAP_FROM {}\n", metalness_from));
        code.push_str("#define METALNESS_MAP_UV_FLIP 0\n");
        code.push_str(&format!("#define METALNESS_MAP_SWIZZLE {}\n", self.metalness_swizzle));
        code.push_str("#define METALNESS_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define METALNESS_MAP_FILE \"{}\"\n\n", self.metalness_file));
        code.push_str(&format!("const float metalness = {:.4};\n", self.metalness_val));
        code.push_str("const float metalnessMapLoopNum = 1.0;\n\n");

        // Specular
        let specular_from = if self.specular_enabled { 3 } else { 0 };
        code.push_str(&format!("#define SPECULAR_MAP_FROM {}\n", specular_from));
        code.push_str("#define SPECULAR_MAP_TYPE 0\n");
        code.push_str("#define SPECULAR_MAP_UV_FLIP 0\n");
        code.push_str("#define SPECULAR_MAP_SWIZZLE 0\n");
        code.push_str("#define SPECULAR_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define SPECULAR_MAP_FILE \"{}\"\n\n", self.specular_file));
        code.push_str(&format!(
            "const float3 specular = float3({:.4}, {:.4}, {:.4});\n",
            self.specular_color[0], self.specular_color[1], self.specular_color[2]
        ));
        code.push_str("const float2 specularMapLoopNum = 1.0;\n\n");

        // Occlusion
        let occlusion_from = if self.occlusion_enabled { 3 } else { 0 };
        code.push_str(&format!("#define OCCLUSION_MAP_FROM {}\n", occlusion_from));
        code.push_str("#define OCCLUSION_MAP_TYPE 0\n");
        code.push_str("#define OCCLUSION_MAP_UV_FLIP 0\n");
        code.push_str(&format!("#define OCCLUSION_MAP_SWIZZLE {}\n", self.occlusion_swizzle));
        code.push_str("#define OCCLUSION_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define OCCLUSION_MAP_FILE \"{}\"\n\n", self.occlusion_file));
        code.push_str(&format!("const float occlusion = {:.4};\n", self.occlusion_val));
        code.push_str("const float occlusionMapLoopNum = 1.0;\n\n");

        // Parallax
        let parallax_from = if self.parallax_enabled { 3 } else { 0 };
        code.push_str(&format!("#define PARALLAX_MAP_FROM {}\n", parallax_from));
        code.push_str("#define PARALLAX_MAP_TYPE 0\n");
        code.push_str("#define PARALLAX_MAP_UV_FLIP 0\n");
        code.push_str("#define PARALLAX_MAP_SWIZZLE 0\n");
        code.push_str(&format!("#define PARALLAX_MAP_FILE \"{}\"\n\n", self.parallax_file));
        code.push_str(&format!("const float parallaxMapScale = {:.4};\n", self.parallax_scale));
        code.push_str("const float parallaxMapLoopNum = 1.0;\n\n");

        // Emissive
        let emissive_en = if self.emissive_enabled { 1 } else { 0 };
        let emissive_from = if self.emissive_enabled { 3 } else { 0 };
        code.push_str(&format!("#define EMISSIVE_ENABLE {}\n", emissive_en));
        code.push_str(&format!("#define EMISSIVE_MAP_FROM {}\n", emissive_from));
        code.push_str("#define EMISSIVE_MAP_UV_FLIP 0\n");
        code.push_str("#define EMISSIVE_MAP_APPLY_SCALE 0\n");
        code.push_str("#define EMISSIVE_MAP_APPLY_MORPH_COLOR 0\n");
        code.push_str("#define EMISSIVE_MAP_APPLY_MORPH_INTENSITY 0\n");
        code.push_str(&format!("#define EMISSIVE_MAP_APPLY_BLINK {}\n", self.emissive_blink_mode));
        code.push_str(&format!("#define EMISSIVE_MAP_FILE \"{}\"\n\n", self.emissive_file));
        code.push_str(&format!(
            "const float3 emissive = float3({:.4}, {:.4}, {:.4});\n",
            self.emissive_color[0], self.emissive_color[1], self.emissive_color[2]
        ));
        code.push_str(&format!(
            "const float3 emissiveBlink = float3({:.4}, {:.4}, {:.4});\n",
            self.emissive_blink[0], self.emissive_blink[1], self.emissive_blink[2]
        ));
        code.push_str(&format!("const float emissiveIntensity = {:.4};\n", self.emissive_intensity));
        code.push_str("const float2 emissiveMapLoopNum = 1.0;\n\n");

        // Custom (Shading Model ID: 0=Default, 1=Skin, 3=Aniso, 4=Glass, 5=Cloth, 6=ClearCoat, 7=Subsurface, 8=Cel, 9=Toon)
        let custom_id = if self.custom_enabled { self.shading_model_id } else { 0 };
        code.push_str(&format!("#define CUSTOM_ENABLE {}\n\n", custom_id));
        let custom_a_from = if self.custom_a_enabled { 3 } else { 0 };
        code.push_str(&format!("#define CUSTOM_A_MAP_FROM {}\n", custom_a_from));
        code.push_str("#define CUSTOM_A_MAP_UV_FLIP 0\n");
        code.push_str("#define CUSTOM_A_MAP_COLOR_FLIP 0\n");
        code.push_str("#define CUSTOM_A_MAP_SWIZZLE 0\n");
        code.push_str("#define CUSTOM_A_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define CUSTOM_A_MAP_FILE \"{}\"\n\n", self.custom_a_file));
        code.push_str(&format!("const float customA = {:.4};\n", self.custom_a_val));
        code.push_str("const float customAMapLoopNum = 1.0;\n\n");

        let custom_b_from = if self.custom_b_enabled { 3 } else { 0 };
        code.push_str(&format!("#define CUSTOM_B_MAP_FROM {}\n", custom_b_from));
        code.push_str("#define CUSTOM_B_MAP_UV_FLIP 0\n");
        code.push_str("#define CUSTOM_B_MAP_COLOR_FLIP 0\n");
        code.push_str("#define CUSTOM_B_MAP_APPLY_SCALE 0\n");
        code.push_str(&format!("#define CUSTOM_B_MAP_FILE \"{}\"\n\n", self.custom_b_file));
        code.push_str(&format!(
            "const float3 customB = float3({:.4}, {:.4}, {:.4});\n",
            self.custom_b_color[0], self.custom_b_color[1], self.custom_b_color[2]
        ));
        code.push_str("const float2 customBMapLoopNum = 1.0;\n\n");

        // Hex-Tiling (Mikkelsen 2022)
        let hex_en = if self.hex_tiling_enable { 1 } else { 0 };
        let hex_lod_en = if self.hex_tiling_distance_lod { 1 } else { 0 };
        code.push_str("// Hex-Tiling (Mikkelsen 2022) - Stochastic seamless texture repetition\n");
        code.push_str(&format!("#define HEX_TILING_ENABLE {}\n", hex_en));
        code.push_str(&format!("#define HEX_TILING_ROTATION {:.2}f\n", self.hex_tiling_rotation));
        code.push_str(&format!("#define HEX_TILING_CONTRAST {:.2}f\n", self.hex_tiling_contrast));
        code.push_str(&format!("#define HEX_TILING_SHARPNESS {:.2}f\n", self.hex_tiling_sharpness));
        code.push_str(&format!("#define HEX_TILING_DISTANCE_LOD {}\n", hex_lod_en));
        code.push_str(&format!("#define HEX_TILING_LOD_START {:.2}f\n", self.hex_tiling_lod_start));
        code.push_str(&format!("#define HEX_TILING_LOD_END {:.2}f\n\n", self.hex_tiling_lod_end));

        // Hashed Alpha Test
        let hashed_en = if self.hashed_alpha_enable { 1 } else { 0 };
        code.push_str("// Hashed Alpha Testing (Wyman & McGuire 2017) - Soft anti-aliased hair/foliage cutouts with TAA\n");
        code.push_str(&format!("#define HASHED_ALPHA_TEST_ENABLE {}\n", hashed_en));
        code.push_str(&format!("#define HASHED_ALPHA_SCALE {:.2}f\n\n", self.hashed_alpha_scale));

        // Detail Normal Map
        let detail_en = if self.detail_map_enable { 1 } else { 0 };
        code.push_str("// Detail Normal Map - Micro-surface relief (skin pores, fabric weave, stone grain)\n");
        code.push_str(&format!("#define DETAIL_MAP_ENABLE {}\n", detail_en));
        code.push_str(&format!("#define DETAIL_NORMAL_MAP_FILE \"{}\"\n", self.detail_normal_file));
        code.push_str(&format!("const float detailNormalScale = {:.4};\n", self.detail_normal_scale));
        code.push_str(&format!("const float detailNormalLoopNum = {:.4};\n", self.detail_normal_loop));
        code.push_str(&format!("const float detailFadeDistance = {:.4};\n\n", self.detail_fade_distance));

        // Common include
        code.push_str("#include \"material_common_2.0.fxsub\"\n");

        code
    }

    /// Exports the material configuration and associated baked textures into target directory.
    pub fn export_package(
        &self,
        target_dir: &Path,
        textures: &[(String, &U8Image)],
    ) -> Result<()> {
        std::fs::create_dir_all(target_dir)
            .with_context(|| format!("Failed to create material directory {:?}", target_dir))?;

        // Save material.fx
        let fx_path = target_dir.join("material.fx");
        let mut fx_file = File::create(&fx_path)
            .with_context(|| format!("Failed to create material.fx at {:?}", fx_path))?;
        fx_file.write_all(self.generate_fx_code().as_bytes())?;

        // Save textures
        for (filename, img) in textures {
            let tex_path = target_dir.join(filename);
            img.save(&tex_path)
                .with_context(|| format!("Failed to save texture {:?}", tex_path))?;
        }

        Ok(())
    }
}
