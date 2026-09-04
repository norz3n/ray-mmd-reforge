//! Real-time PBR material previewer with Cook-Torrance GGX BRDF and interactive camera.
//!
//! Renders an interactive 3D material sphere, cube, or flat plane with:
//! - Tangent-space Normal Mapping (with TBN calculation)
//! - Physically Based Rendering: Cook-Torrance GGX Specular, Disney/Lambert Diffuse, Schlick Fresnel
//! - Metalness / Roughness workflow matching Ray-MMD ReForge
//! - Ambient Occlusion, Specular F0, and Emissive glow
//! - Interactive orbit camera and light rotation

use glam::{Mat3, Vec3};
use rayon::prelude::*;
use crate::graph::eval::EvaluatedMaterial;
use crate::image_proc::U8Image;

/// 3D Geometry mesh primitive for material preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PreviewPrimitive {
    Sphere,
    Cube,
    Plane,
}

/// Display channel mode for 3D viewport inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ViewportDisplayMode {
    #[default]
    FullPbr,
    AlbedoOnly,
    NormalOnly,
    RoughnessOnly,
    MetalnessOnly,
    OcclusionOnly,
    EmissiveOnly,
    CustomAOnly,
    CustomBOnly,
}

impl ViewportDisplayMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FullPbr => "🌐 Full PBR Preview",
            Self::AlbedoOnly => "🎨 Albedo (Base Color)",
            Self::NormalOnly => "🧭 Normal Map",
            Self::RoughnessOnly => "⚪ Roughness",
            Self::MetalnessOnly => "🪙 Metalness",
            Self::OcclusionOnly => "🌑 Ambient Occlusion",
            Self::EmissiveOnly => "💡 Emissive Glow",
            Self::CustomAOnly => "🎭 Custom A Channel",
            Self::CustomBOnly => "🎭 Custom B Channel",
        }
    }
}

/// Camera and lighting parameters for the preview viewport.
#[derive(Debug, Clone)]
pub struct PreviewCamera {
    pub target: glam::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub light_yaw: f32,
    pub light_pitch: f32,
    pub light_intensity: f32,
    pub primitive: PreviewPrimitive,
    pub display_mode: ViewportDisplayMode,
}

impl Default for PreviewCamera {
    fn default() -> Self {
        Self {
            target: glam::Vec3::ZERO,
            yaw: 0.5,
            pitch: 0.3,
            distance: 2.2,
            light_yaw: 1.0,
            light_pitch: 0.8,
            light_intensity: 1.5,
            primitive: PreviewPrimitive::Sphere,
            display_mode: ViewportDisplayMode::FullPbr,
        }
    }
}

/// Helper to bilinear sample an image at UV coordinate (0.0 - 1.0) with wrap.
#[inline(always)]
fn sample_texture_bilinear(img: Option<&U8Image>, u: f32, v: f32, default: [f32; 4]) -> [f32; 4] {
    let img = match img {
        Some(m) if m.width() > 0 && m.height() > 0 => m,
        _ => return default,
    };

    let w = img.width() as f32;
    let h = img.height() as f32;

    // Wrap UV to [0.0, 1.0)
    let u_wrapped = u.fract().rem_euclid(1.0);
    let v_wrapped = v.fract().rem_euclid(1.0);

    let fx = u_wrapped * w - 0.5;
    let fy = v_wrapped * h - 0.5;

    let x0 = fx.floor().rem_euclid(w) as usize;
    let y0 = fy.floor().rem_euclid(h) as usize;
    let x1 = (x0 + 1) % img.width() as usize;
    let y1 = (y0 + 1) % img.height() as usize;

    let wx = fx.fract().rem_euclid(1.0);
    let wy = fy.fract().rem_euclid(1.0);

    let raw = img.as_raw();
    let stride = img.width() as usize * 4;
    let i00 = y0 * stride + x0 * 4;
    let i10 = y0 * stride + x1 * 4;
    let i01 = y1 * stride + x0 * 4;
    let i11 = y1 * stride + x1 * 4;

    let mut out = [0.0f32; 4];
    for c in 0..4 {
        let v00 = raw[i00 + c] as f32 * (1.0 / 255.0);
        let v10 = raw[i10 + c] as f32 * (1.0 / 255.0);
        let v01 = raw[i01 + c] as f32 * (1.0 / 255.0);
        let v11 = raw[i11 + c] as f32 * (1.0 / 255.0);

        let top = v00 * (1.0 - wx) + v10 * wx;
        let bot = v01 * (1.0 - wx) + v11 * wx;
        out[c] = top * (1.0 - wy) + bot * wy;
    }
    out
}

/// Renders the 3D material preview to an RGBA pixel buffer.
pub fn render_pbr_preview(
    material: &EvaluatedMaterial,
    camera: &PreviewCamera,
    width: u32,
    height: u32,
) -> U8Image {
    let mut out = U8Image::new(width, height);
    let w_f = width as f32;
    let h_f = height as f32;
    let aspect = w_f / h_f;

    // Camera setup
    let cam_x = camera.distance * camera.pitch.cos() * camera.yaw.sin();
    let cam_y = camera.distance * camera.pitch.sin();
    let cam_z = camera.distance * camera.pitch.cos() * camera.yaw.cos();
    let cam_pos = Vec3::new(cam_x, cam_y, cam_z);
    let target = Vec3::ZERO;
    let forward = (target - cam_pos).normalize();
    let right = forward.cross(Vec3::Y).normalize();
    let up = right.cross(forward).normalize();

    // Key light direction
    let light_dir = Vec3::new(
        camera.light_pitch.cos() * camera.light_yaw.sin(),
        camera.light_pitch.sin(),
        camera.light_pitch.cos() * camera.light_yaw.cos(),
    )
    .normalize();

    let light_color = Vec3::ONE * camera.light_intensity;
    let ambient_sky = Vec3::new(0.2, 0.25, 0.35) * 0.4;
    let ambient_ground = Vec3::new(0.12, 0.1, 0.08) * 0.4;

    out.as_flat_samples_mut()
        .as_mut_slice()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let px = (idx % width as usize) as f32;
            let py = (idx / width as usize) as f32;

            // Normalized Device Coordinates [-1, 1]
            let ndc_x = (2.0 * (px + 0.5) / w_f - 1.0) * aspect;
            let ndc_y = 1.0 - 2.0 * (py + 0.5) / h_f;

            // Perspective Ray
            let fov_factor = 1.0; // ~53 degree FOV
            let ray_dir = (forward + right * (ndc_x * fov_factor) + up * (ndc_y * fov_factor)).normalize();

            let hit = match camera.primitive {
                PreviewPrimitive::Sphere => intersect_sphere(cam_pos, ray_dir, Vec3::ZERO, 1.0),
                PreviewPrimitive::Cube => intersect_cube(cam_pos, ray_dir, 1.4),
                PreviewPrimitive::Plane => intersect_plane(cam_pos, ray_dir),
            };

            if let Some((hit_pos, geo_normal, uv, tangent, bitangent)) = hit {
                let view_dir = (cam_pos - hit_pos).normalize();

                // Sample textures
                let albedo_samp = sample_texture_bilinear(material.albedo.as_ref(), uv.x, uv.y, [0.8, 0.8, 0.8, 1.0]);
                let albedo = Vec3::new(albedo_samp[0], albedo_samp[1], albedo_samp[2]);

                // Normal map perturbation
                let normal_samp = sample_texture_bilinear(material.normal.as_ref(), uv.x, uv.y, [0.5, 0.5, 1.0, 1.0]);
                let map_n = Vec3::new(
                    normal_samp[0] * 2.0 - 1.0,
                    normal_samp[1] * 2.0 - 1.0, // DirectX Y-
                    normal_samp[2] * 2.0 - 1.0,
                )
                .normalize();

                // TBN transformation
                let tbn = Mat3::from_cols(tangent, bitangent, geo_normal);
                let normal = (tbn * map_n).normalize();

                // Roughness & Metalness
                let rough_samp = sample_texture_bilinear(material.smoothness.as_ref(), uv.x, uv.y, [0.5, 0.5, 0.5, 1.0]);
                let roughness = (1.0 - rough_samp[0]).clamp(0.04, 1.0); // GGX alpha
                let alpha = roughness * roughness;

                let metal_samp = sample_texture_bilinear(material.metalness.as_ref(), uv.x, uv.y, [0.0, 0.0, 0.0, 1.0]);
                let metalness = metal_samp[0].clamp(0.0, 1.0);

                let ao_samp = sample_texture_bilinear(material.occlusion.as_ref(), uv.x, uv.y, [1.0, 1.0, 1.0, 1.0]);
                let ao = ao_samp[0].clamp(0.0, 1.0);

                let emissive_samp = sample_texture_bilinear(material.emissive.as_ref(), uv.x, uv.y, [0.0, 0.0, 0.0, 1.0]);
                let emissive = Vec3::new(emissive_samp[0], emissive_samp[1], emissive_samp[2]);

                let custom_a_samp = sample_texture_bilinear(material.custom_a.as_ref(), uv.x, uv.y, [0.5, 0.5, 0.5, 1.0]);
                let custom_a = custom_a_samp[0];

                let custom_b_samp = sample_texture_bilinear(material.custom_b.as_ref(), uv.x, uv.y, [1.0, 1.0, 1.0, 1.0]);
                let custom_b = Vec3::new(custom_b_samp[0], custom_b_samp[1], custom_b_samp[2]);

                // Viewport Channel Isolation
                match camera.display_mode {
                    ViewportDisplayMode::AlbedoOnly => {
                        out_pix[0] = (albedo.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[1] = (albedo.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[2] = (albedo.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::NormalOnly => {
                        let n_vis = normal * 0.5 + Vec3::splat(0.5);
                        out_pix[0] = (n_vis.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[1] = (n_vis.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[2] = (n_vis.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::RoughnessOnly => {
                        let b = (roughness * 255.0 + 0.5) as u8;
                        out_pix[0] = b; out_pix[1] = b; out_pix[2] = b; out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::MetalnessOnly => {
                        let b = (metalness * 255.0 + 0.5) as u8;
                        out_pix[0] = b; out_pix[1] = b; out_pix[2] = b; out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::OcclusionOnly => {
                        let b = (ao * 255.0 + 0.5) as u8;
                        out_pix[0] = b; out_pix[1] = b; out_pix[2] = b; out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::EmissiveOnly => {
                        out_pix[0] = (emissive.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[1] = (emissive.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[2] = (emissive.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::CustomAOnly => {
                        let b = (custom_a.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[0] = b; out_pix[1] = b; out_pix[2] = b; out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::CustomBOnly => {
                        out_pix[0] = (custom_b.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[1] = (custom_b.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[2] = (custom_b.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                        out_pix[3] = 255;
                        return;
                    }
                    ViewportDisplayMode::FullPbr => {}
                }

                // PBR BRDF Evaluation
                let f0 = Vec3::splat(0.04).lerp(albedo, metalness);
                let h = (light_dir + view_dir).normalize();

                let n_dot_l = normal.dot(light_dir).max(0.0);
                let n_dot_v = normal.dot(view_dir).max(1e-4);
                let n_dot_h = normal.dot(h).max(0.0);
                let v_dot_h = view_dir.dot(h).max(0.0);

                // GGX Normal Distribution Function D
                let alpha2 = alpha * alpha;
                let d_denom = n_dot_h * n_dot_h * (alpha2 - 1.0) + 1.0;
                let d = alpha2 / (std::f32::consts::PI * d_denom * d_denom);

                // Schlick Fresnel F
                let f = f0 + (Vec3::ONE - f0) * (1.0 - v_dot_h).powi(5);

                // Smith Masking-Shadowing G
                let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
                let g1_l = n_dot_l / (n_dot_l * (1.0 - k) + k);
                let g1_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
                let g = g1_l * g1_v;

                let specular_brdf = (d * g * f) / (4.0 * n_dot_l * n_dot_v).max(1e-4);
                let kd = (Vec3::ONE - f) * (1.0 - metalness);
                let diffuse_brdf = kd * albedo / std::f32::consts::PI;

                let direct_lighting = (diffuse_brdf + specular_brdf) * light_color * n_dot_l;

                // Ambient lighting approximation
                let up_factor = normal.y * 0.5 + 0.5;
                let ambient_light = ambient_ground.lerp(ambient_sky, up_factor);
                let ambient = (kd * albedo + f0 * (1.0 - roughness)) * ambient_light * ao;

                // Custom A & Custom B lighting contributions
                let mut custom_lighting = Vec3::ZERO;
                if material.custom_a.is_some() {
                    let cc_rough = (1.0 - custom_a).clamp(0.04, 1.0);
                    let cc_alpha = cc_rough * cc_rough;
                    let cc_d = cc_alpha / (std::f32::consts::PI * (n_dot_h * n_dot_h * (cc_alpha - 1.0) + 1.0).powi(2));
                    let cc_f = 0.04 + (1.0 - 0.04) * (1.0 - v_dot_h).powi(5);
                    let cc_spec = cc_d * cc_f / (4.0 * n_dot_l * n_dot_v).max(1e-4);
                    custom_lighting += Vec3::splat(cc_spec * n_dot_l * 0.5);
                }
                if material.custom_b.is_some() {
                    let scatter = (1.0 - n_dot_l) * custom_a * 0.25;
                    custom_lighting += custom_b * albedo * scatter;
                }

                let total_color = direct_lighting + ambient + emissive + custom_lighting;

                // Reinhard tonemapping + gamma correction
                let tonemapped = total_color / (total_color + Vec3::ONE);
                let gamma_corrected = tonemapped.powf(1.0 / 2.2);

                out_pix[0] = (gamma_corrected.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                out_pix[1] = (gamma_corrected.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                out_pix[2] = (gamma_corrected.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                out_pix[3] = 255;
            } else {
                // Background dark studio gradient
                let grad = (py / h_f).clamp(0.0, 1.0);
                let bg_val = (0.12 - grad * 0.05).clamp(0.0, 1.0);
                let bg_byte = (bg_val * 255.0 + 0.5) as u8;

                out_pix[0] = bg_byte;
                out_pix[1] = (bg_byte as f32 * 1.05).min(255.0) as u8;
                out_pix[2] = (bg_byte as f32 * 1.15).min(255.0) as u8;
                out_pix[3] = 255;
            }
        });

    out
}

/// Ray-sphere intersection with analytical TBN derivation.
fn intersect_sphere(
    ro: Vec3,
    rd: Vec3,
    center: Vec3,
    radius: f32,
) -> Option<(Vec3, Vec3, glam::Vec2, Vec3, Vec3)> {
    let oc = ro - center;
    let b = oc.dot(rd);
    let c = oc.dot(oc) - radius * radius;
    let h = b * b - c;

    if h < 0.0 {
        return None;
    }

    let t = -b - h.sqrt();
    if t < 0.0 {
        return None;
    }

    let pos = ro + rd * t;
    let normal = (pos - center) / radius;

    // Equirectangular spherical UV mapping
    let u = normal.x.atan2(normal.z) / std::f32::consts::TAU + 0.5;
    let v = normal.y.clamp(-1.0, 1.0).asin() / std::f32::consts::PI + 0.5;

    // Analytical sphere tangent & bitangent
    let tangent = Vec3::new(normal.z, 0.0, -normal.x).normalize_or_zero();
    let bitangent = normal.cross(tangent).normalize_or_zero();

    Some((pos, normal, glam::Vec2::new(u, v), tangent, bitangent))
}

/// Ray-cube intersection.
fn intersect_cube(
    ro: Vec3,
    rd: Vec3,
    size: f32,
) -> Option<(Vec3, Vec3, glam::Vec2, Vec3, Vec3)> {
    let half = size * 0.5;
    let min_b = Vec3::splat(-half);
    let max_b = Vec3::splat(half);

    let inv_d = Vec3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);
    let t0 = (min_b - ro) * inv_d;
    let t1 = (max_b - ro) * inv_d;

    let t_min = t0.min(t1);
    let t_max = t0.max(t1);

    let near = t_min.x.max(t_min.y).max(t_min.z);
    let far = t_max.x.min(t_max.y).min(t_max.z);

    if near > far || far < 0.0 {
        return None;
    }

    let t = if near > 0.0 { near } else { far };
    let pos = ro + rd * t;
    let eps = 1e-3;

    let (normal, uv, tangent, bitangent) = if (pos.x - half).abs() < eps {
        (Vec3::X, glam::Vec2::new((pos.z + half) / size, (pos.y + half) / size), Vec3::Z, Vec3::Y)
    } else if (pos.x + half).abs() < eps {
        (-Vec3::X, glam::Vec2::new((-pos.z + half) / size, (pos.y + half) / size), -Vec3::Z, Vec3::Y)
    } else if (pos.y - half).abs() < eps {
        (Vec3::Y, glam::Vec2::new((pos.x + half) / size, (pos.z + half) / size), Vec3::X, Vec3::Z)
    } else if (pos.y + half).abs() < eps {
        (-Vec3::Y, glam::Vec2::new((pos.x + half) / size, (-pos.z + half) / size), Vec3::X, -Vec3::Z)
    } else if (pos.z - half).abs() < eps {
        (Vec3::Z, glam::Vec2::new((-pos.x + half) / size, (pos.y + half) / size), -Vec3::X, Vec3::Y)
    } else {
        (-Vec3::Z, glam::Vec2::new((pos.x + half) / size, (pos.y + half) / size), Vec3::X, Vec3::Y)
    };

    Some((pos, normal, uv, tangent, bitangent))
}

/// Ray-plane intersection.
fn intersect_plane(
    ro: Vec3,
    rd: Vec3,
) -> Option<(Vec3, Vec3, glam::Vec2, Vec3, Vec3)> {
    let normal = Vec3::Y;
    let denom = normal.dot(rd);
    if denom.abs() < 1e-4 {
        return None;
    }

    let t = -normal.dot(ro) / denom;
    if t < 0.0 {
        return None;
    }

    let pos = ro + rd * t;
    let size = 2.0;
    if pos.x.abs() > size || pos.z.abs() > size {
        return None;
    }

    let uv = glam::Vec2::new((pos.x / (2.0 * size)) + 0.5, (pos.z / (2.0 * size)) + 0.5);
    let tangent = Vec3::X;
    let bitangent = Vec3::Z;

    Some((pos, normal, uv, tangent, bitangent))
}
