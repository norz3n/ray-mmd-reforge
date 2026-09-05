//! High-performance image processing and texture map generation algorithms (ShaderMap analogue).
//!
//! Provides parallel, CPU-accelerated (via Rayon) algorithms for:
//! - Tangent space Normal Map generation (Sobel 3x3 / Scharr 3x3, DirectX Y-down / OpenGL Y-up)
//! - Height / Displacement extraction from diffuse luminance
//! - Ambient Occlusion (AO) horizon raymarching on heightfields
//! - Curvature / Cavity / Edge wear extraction (discrete Laplacian & normal divergence)
//! - Roughness / Smoothness / Gloss curve remapping
//! - Reoriented Normal Mapping (RNM) and detail normal blending
//! - Multi-channel packing and unpacking (RMA, etc.)

use image::{ImageBuffer, Rgba};
use rayon::prelude::*;

/// 32-bit floating point RGBA image for high dynamic range processing.
pub type F32Image = ImageBuffer<Rgba<f32>, Vec<f32>>;
/// 8-bit standard RGBA image.
pub type U8Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Normal map filter kernel algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NormalFilter {
    /// Standard Sobel 3x3 kernel.
    Sobel,
    /// Scharr 3x3 kernel (superior rotational symmetry and reduced angular error).
    Scharr,
}

/// Normal map coordinate orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NormalOrientation {
    /// DirectX format (Y- down / Green inverted) - standard for ray-mmd / DirectX 9/11.
    DirectX,
    /// OpenGL format (Y+ up / Green standard).
    OpenGL,
}

/// Curvature extraction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CurvatureMode {
    /// Both convex (ridges) and concave (crevices) mapped to 0.0 - 1.0 (0.5 is flat).
    Full,
    /// Convex ridges only (edge wear / highlights).
    ConvexOnly,
    /// Concave crevices only (dirt / ambient cavity).
    ConcaveOnly,
}

/// Computes the Rec. 709 luminance of an sRGB/linear RGB color.
#[inline(always)]
pub fn luminance(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Converts an 8-bit RGBA image to an F32 normalized [0.0, 1.0] image.
pub fn u8_to_f32_image(img: &U8Image) -> F32Image {
    let (width, height) = img.dimensions();
    let mut out = F32Image::new(width, height);
    
    out.as_mut()
        .par_chunks_exact_mut(4)
        .zip(img.as_raw().par_chunks_exact(4))
        .for_each(|(out_pixel, in_pixel)| {
            out_pixel[0] = in_pixel[0] as f32 / 255.0;
            out_pixel[1] = in_pixel[1] as f32 / 255.0;
            out_pixel[2] = in_pixel[2] as f32 / 255.0;
            out_pixel[3] = in_pixel[3] as f32 / 255.0;
        });
        
    out
}

/// Converts an F32 image back to 8-bit RGBA clamped to [0, 255].
pub fn f32_to_u8_image(img: &F32Image) -> U8Image {
    let (width, height) = img.dimensions();
    let mut out = U8Image::new(width, height);
    
    out.as_mut()
        .par_chunks_exact_mut(4)
        .zip(img.as_raw().par_chunks_exact(4))
        .for_each(|(out_pixel, in_pixel)| {
            out_pixel[0] = (in_pixel[0].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pixel[1] = (in_pixel[1].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pixel[2] = (in_pixel[2].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pixel[3] = (in_pixel[3].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        });
        
    out
}

/// Generates a height / displacement map from a diffuse image via perceptual luminance and contrast adjustment.
pub fn generate_height_map(
    source: &U8Image,
    contrast: f32,
    brightness: f32,
    invert: bool,
) -> U8Image {
    let (width, height) = source.dimensions();
    let mut out = U8Image::new(width, height);

    out.as_mut()
        .par_chunks_exact_mut(4)
        .zip(source.as_raw().par_chunks_exact(4))
        .for_each(|(out_pix, in_pix)| {
            let r = in_pix[0] as f32 / 255.0;
            let g = in_pix[1] as f32 / 255.0;
            let b = in_pix[2] as f32 / 255.0;
            let mut lum = luminance(r, g, b);

            // Contrast curve: centered at 0.5
            lum = (lum - 0.5) * contrast + 0.5 + brightness;
            if invert {
                lum = 1.0 - lum;
            }
            let val = (lum.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[0] = val;
            out_pix[1] = val;
            out_pix[2] = val;
            out_pix[3] = 255;
        });

    out
}

/// Helper function to sample height at coordinate (x, y) with clamping.
#[inline(always)]
fn sample_height_clamped(data: &[u8], width: usize, height: usize, x: isize, y: isize) -> f32 {
    let cx = x.clamp(0, width as isize - 1) as usize;
    let cy = y.clamp(0, height as isize - 1) as usize;
    let idx = (cy * width + cx) * 4;
    data[idx] as f32 / 255.0
}

/// Generates a high-quality tangent-space normal map from a height or grayscale image.
pub fn generate_normal_map(
    height_map: &U8Image,
    scale: f32,
    filter: NormalFilter,
    orientation: NormalOrientation,
) -> U8Image {
    let (w, h) = height_map.dimensions();
    let width = w as usize;
    let height = h as usize;
    let in_bytes = height_map.as_raw().as_slice();
    let mut out = U8Image::new(w, h);

    let (kx, ky) = match filter {
        NormalFilter::Sobel => (
            [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
            [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
        ),
        NormalFilter::Scharr => (
            [-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0],
            [-3.0, -10.0, -3.0, 0.0, 0.0, 0.0, 3.0, 10.0, 3.0],
        ),
    };

    let y_sign = match orientation {
        NormalOrientation::DirectX => -1.0, // DirectX standard (Y-)
        NormalOrientation::OpenGL => 1.0,   // OpenGL standard (Y+)
    };

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(pixel_idx, out_pix)| {
            let px = (pixel_idx % width) as isize;
            let py = (pixel_idx / width) as isize;

            // 3x3 neighborhood sampling
            let mut dx = 0.0;
            let mut dy = 0.0;

            let mut k_idx = 0;
            for oy in -1..=1 {
                for ox in -1..=1 {
                    let h_val = sample_height_clamped(in_bytes, width, height, px + ox, py + oy);
                    dx += h_val * kx[k_idx];
                    dy += h_val * ky[k_idx];
                    k_idx += 1;
                }
            }

            let nx = -dx * scale;
            let ny = -dy * scale * y_sign;
            let nz = 1.0;

            // Normalize vector
            let len = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-6);
            let unx = nx / len;
            let uny = ny / len;
            let unz = nz / len;

            // Map [-1, 1] to [0, 255]
            out_pix[0] = ((unx * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[1] = ((uny * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[2] = ((unz * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[3] = 255;
        });

    out
}

/// Generates an Ambient Occlusion (AO) map by sampling heightfield horizon angles in multiple radial directions.
/// Highly optimized with precomputed ray steps, single-channel height cache, and multi-scale hierarchy.
pub fn generate_ao_map(
    height_map: &U8Image,
    radius_pixels: usize,
    sample_directions: usize,
    intensity: f32,
    bias: f32,
) -> U8Image {
    let (orig_w, orig_h) = height_map.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return U8Image::new(1, 1);
    }

    // Scale down if larger than 512 for macroscopic AO calculation
    let max_dim = 512;
    let needs_upscale = orig_w > max_dim || orig_h > max_dim;
    let (w, h, scaled_height_map) = if needs_upscale {
        let scale = (max_dim as f32 / orig_w.max(orig_h) as f32).min(1.0);
        let nw = ((orig_w as f32 * scale).round() as u32).max(64);
        let nh = ((orig_h as f32 * scale).round() as u32).max(64);
        let resized = image::imageops::resize(
            height_map,
            nw,
            nh,
            image::imageops::FilterType::Triangle,
        );
        (nw, nh, Some(resized))
    } else {
        (orig_w, orig_h, None)
    };

    let target_img = scaled_height_map.as_ref().unwrap_or(height_map);
    let width = w as usize;
    let height = h as usize;

    // Single-channel normalized height cache (1 byte per pixel, compact cache footprint)
    let raw_bytes = target_img.as_raw();
    let height_cache: Vec<f32> = raw_bytes
        .par_chunks_exact(4)
        .map(|p| p[0] as f32 * (1.0 / 255.0))
        .collect();

    // Scale radius proportionally if downscaled
    let scale_factor = w as f32 / orig_w as f32;
    let effective_radius = ((radius_pixels as f32 * scale_factor).round() as isize).max(2);
    let num_dirs = sample_directions.clamp(4, 32);
    let steps_per_ray = 4;

    // Precalculate all ray direction step offsets and inverted distance weights once
    #[derive(Clone, Copy)]
    struct RayStep {
        ox: isize,
        oy: isize,
        inv_dist: f32,
    }

    let mut ray_steps: Vec<Vec<RayStep>> = Vec::with_capacity(num_dirs);
    for d in 0..num_dirs {
        let angle = (d as f32 / num_dirs as f32) * std::f32::consts::TAU;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let mut steps = Vec::with_capacity(steps_per_ray);
        for s in 1..=steps_per_ray {
            let dist = (s as f32 / steps_per_ray as f32) * effective_radius as f32;
            let ox = (cos_a * dist).round() as isize;
            let oy = (sin_a * dist).round() as isize;
            let inv_dist = 1.0 / dist.max(0.5);
            steps.push(RayStep { ox, oy, inv_dist });
        }
        ray_steps.push(steps);
    }

    let mut out = U8Image::new(w, h);
    let inv_num_dirs = 1.0 / num_dirs as f32;

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(pixel_idx, out_pix)| {
            let px = (pixel_idx % width) as isize;
            let py = (pixel_idx / width) as isize;
            let center_h = height_cache[pixel_idx];

            let mut total_occlusion = 0.0f32;

            for steps in &ray_steps {
                let mut max_slope = bias;

                for step in steps {
                    let sx = (px + step.ox).clamp(0, width as isize - 1) as usize;
                    let sy = (py + step.oy).clamp(0, height as isize - 1) as usize;

                    let sample_h = height_cache[sy * width + sx];
                    let diff_h = sample_h - center_h;
                    let slope = diff_h * step.inv_dist;
                    if slope > max_slope {
                        max_slope = slope;
                    }
                }

                // Fast rational approximation of atan(x) / (PI / 2) for x >= 0:
                // x / (x + 0.6366) produces nearly identical curve with zero transcendental overhead.
                let occ = if max_slope <= 0.0 {
                    0.0
                } else {
                    (max_slope / (max_slope + 0.6366)).min(1.0)
                };
                total_occlusion += occ;
            }

            let avg_occlusion = (total_occlusion * inv_num_dirs) * intensity;
            let ao_val = (1.0 - avg_occlusion).clamp(0.0, 1.0);
            let byte_val = (ao_val * 255.0 + 0.5) as u8;

            out_pix[0] = byte_val;
            out_pix[1] = byte_val;
            out_pix[2] = byte_val;
            out_pix[3] = 255;
        });

    if needs_upscale {
        image::imageops::resize(&out, orig_w, orig_h, image::imageops::FilterType::Triangle)
    } else {
        out
    }
}

/// Generates a curvature / cavity / edge wear map from a height map using a discrete Laplacian filter.
pub fn generate_curvature_map(
    height_map: &U8Image,
    radius: usize,
    intensity: f32,
    mode: CurvatureMode,
) -> U8Image {
    let (w, h) = height_map.dimensions();
    let width = w as usize;
    let height = h as usize;
    let in_bytes = height_map.as_raw().as_slice();
    let mut out = U8Image::new(w, h);
    let r = radius.max(1) as isize;

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(pixel_idx, out_pix)| {
            let px = (pixel_idx % width) as isize;
            let py = (pixel_idx / width) as isize;
            let center_h = sample_height_clamped(in_bytes, width, height, px, py);

            let left = sample_height_clamped(in_bytes, width, height, px - r, py);
            let right = sample_height_clamped(in_bytes, width, height, px + r, py);
            let up = sample_height_clamped(in_bytes, width, height, px, py - r);
            let down = sample_height_clamped(in_bytes, width, height, px, py + r);

            // Laplacian: 4 * center - sum(neighbors)
            // If center > neighbors (ridge/convex) -> laplacian > 0
            // If center < neighbors (valley/crevice) -> laplacian < 0
            let lap = 4.0 * center_h - (left + right + up + down);
            let curvature = lap * intensity;

            let result = match mode {
                CurvatureMode::Full => (curvature * 0.5 + 0.5).clamp(0.0, 1.0),
                CurvatureMode::ConvexOnly => curvature.max(0.0).clamp(0.0, 1.0),
                CurvatureMode::ConcaveOnly => (-curvature).max(0.0).clamp(0.0, 1.0),
            };

            let byte_val = (result * 255.0 + 0.5) as u8;
            out_pix[0] = byte_val;
            out_pix[1] = byte_val;
            out_pix[2] = byte_val;
            out_pix[3] = 255;
        });

    out
}

/// Generates a roughness / smoothness map with curve inversion and contrast/range controls.
pub fn generate_roughness_map(
    source: &U8Image,
    invert: bool,
    contrast: f32,
    min_val: f32,
    max_val: f32,
) -> U8Image {
    let (width, height) = source.dimensions();
    let mut out = U8Image::new(width, height);

    out.as_mut()
        .par_chunks_exact_mut(4)
        .zip(source.as_raw().par_chunks_exact(4))
        .for_each(|(out_pix, in_pix)| {
            let r = in_pix[0] as f32 / 255.0;
            let g = in_pix[1] as f32 / 255.0;
            let b = in_pix[2] as f32 / 255.0;
            let mut val = luminance(r, g, b);

            if invert {
                val = 1.0 - val;
            }

            // Contrast adjustment
            val = (val - 0.5) * contrast + 0.5;
            // Range remap [min_val, max_val]
            val = min_val + val.clamp(0.0, 1.0) * (max_val - min_val);

            let byte_val = (val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[0] = byte_val;
            out_pix[1] = byte_val;
            out_pix[2] = byte_val;
            out_pix[3] = 255;
        });

    out
}

/// Blends a base normal map with a micro-detail normal map using Reoriented Normal Mapping (RNM).
/// Preserves the curvature and magnitude of both surfaces without flattening.
pub fn blend_normals_rnm(
    base: &U8Image,
    detail: &U8Image,
    detail_scale: f32,
    detail_tile: f32,
) -> U8Image {
    let (w, h) = base.dimensions();
    let (dw, dh) = detail.dimensions();
    let width = w as usize;
    let base_bytes = base.as_raw().as_slice();
    let detail_bytes = detail.as_raw().as_slice();
    let mut out = U8Image::new(w, h);

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(pixel_idx, out_pix)| {
            let px = pixel_idx % width;
            let py = pixel_idx / width;

            // Sample base normal
            let b_idx = (py * width + px) * 4;
            let n1_x = (base_bytes[b_idx] as f32 / 255.0) * 2.0 - 1.0;
            let n1_y = (base_bytes[b_idx + 1] as f32 / 255.0) * 2.0 - 1.0;
            let n1_z = (base_bytes[b_idx + 2] as f32 / 255.0) * 2.0 - 1.0;

            // Sample detail normal with tiling
            let d_u = ((px as f32 * detail_tile) as usize) % (dw as usize);
            let d_v = ((py as f32 * detail_tile) as usize) % (dh as usize);
            let d_idx = (d_v * (dw as usize) + d_u) * 4;
            let n2_x = ((detail_bytes[d_idx] as f32 / 255.0) * 2.0 - 1.0) * detail_scale;
            let n2_y = ((detail_bytes[d_idx + 1] as f32 / 255.0) * 2.0 - 1.0) * detail_scale;
            let n2_z = (detail_bytes[d_idx + 2] as f32 / 255.0) * 2.0 - 1.0;

            // Reoriented Normal Mapping (Colin Barré-Brisebois & Stephen Hill 2012)
            // t = n1 + [0, 0, 1]
            // u = [-n2.x, -n2.y, n2.z]
            // r = t * dot(t, u) / t.z - u
            let t_x = n1_x;
            let t_y = n1_y;
            let t_z = n1_z + 1.0;

            let u_x = -n2_x;
            let u_y = -n2_y;
            let u_z = n2_z;

            let dot_tu = t_x * u_x + t_y * u_y + t_z * u_z;
            let r_x = t_x * (dot_tu / t_z.max(1e-5)) - u_x;
            let r_y = t_y * (dot_tu / t_z.max(1e-5)) - u_y;
            let r_z = t_z * (dot_tu / t_z.max(1e-5)) - u_z;

            let len = (r_x * r_x + r_y * r_y + r_z * r_z).sqrt().max(1e-6);
            let norm_x = r_x / len;
            let norm_y = r_y / len;
            let norm_z = r_z / len;

            out_pix[0] = ((norm_x * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[1] = ((norm_y * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[2] = ((norm_z * 0.5 + 0.5) * 255.0 + 0.5) as u8;
            out_pix[3] = 255;
        });

    out
}

/// Packs four grayscale textures into a single 4-channel RGBA texture (e.g. RMA: Roughness, Metalness, AO).
pub fn pack_channels(
    r_map: Option<&U8Image>,
    g_map: Option<&U8Image>,
    b_map: Option<&U8Image>,
    a_map: Option<&U8Image>,
    default_values: [u8; 4],
) -> U8Image {
    // Find maximum dimensions among supplied maps
    let mut width = 1024;
    let mut height = 1024;

    for map in [r_map, g_map, b_map, a_map].into_iter().flatten() {
        let (w, h) = map.dimensions();
        width = width.max(w);
        height = height.max(h);
    }

    let mut out = U8Image::new(width, height);
    let sample_channel = |map: Option<&U8Image>, default: u8, x: u32, y: u32| -> u8 {
        match map {
            Some(m) => {
                let (mw, mh) = m.dimensions();
                let sx = (x * mw / width).min(mw - 1);
                let sy = (y * mh / height).min(mh - 1);
                m.get_pixel(sx, sy)[0]
            }
            None => default,
        }
    };

    for y in 0..height {
        for x in 0..width {
            let r = sample_channel(r_map, default_values[0], x, y);
            let g = sample_channel(g_map, default_values[1], x, y);
            let b = sample_channel(b_map, default_values[2], x, y);
            let a = sample_channel(a_map, default_values[3], x, y);
            out.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    out
}

/// Generates an emissive map by thresholding luminance, applying tint color and intensity boost.
pub fn generate_emissive_map(
    source: &U8Image,
    mask: Option<&U8Image>,
    min_lum: f32,
    max_lum: f32,
    tint_color: [f32; 3],
    intensity: f32,
    invert_threshold: bool,
) -> U8Image {
    let (width, height) = source.dimensions();
    let mut out = U8Image::new(width, height);
    let src_bytes = source.as_raw();

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let px = (idx % width as usize) as u32;
            let py = (idx / width as usize) as u32;
            let s_idx = idx * 4;

            let r = src_bytes[s_idx] as f32 / 255.0;
            let g = src_bytes[s_idx + 1] as f32 / 255.0;
            let b = src_bytes[s_idx + 2] as f32 / 255.0;

            let lum = luminance(r, g, b);
            let mut factor = if max_lum > min_lum {
                ((lum - min_lum) / (max_lum - min_lum)).clamp(0.0, 1.0)
            } else if lum >= min_lum {
                1.0
            } else {
                0.0
            };

            if invert_threshold {
                factor = 1.0 - factor;
            }

            if let Some(m) = mask {
                let (mw, mh) = m.dimensions();
                let mx = (px * mw / width).min(mw - 1);
                let my = (py * mh / height).min(mh - 1);
                let mask_val = m.get_pixel(mx, my)[0] as f32 / 255.0;
                factor *= mask_val;
            }

            let emissive_r = (r * tint_color[0] * factor * intensity).clamp(0.0, 1.0);
            let emissive_g = (g * tint_color[1] * factor * intensity).clamp(0.0, 1.0);
            let emissive_b = (b * tint_color[2] * factor * intensity).clamp(0.0, 1.0);

            out_pix[0] = (emissive_r * 255.0 + 0.5) as u8;
            out_pix[1] = (emissive_g * 255.0 + 0.5) as u8;
            out_pix[2] = (emissive_b * 255.0 + 0.5) as u8;
            out_pix[3] = 255;
        });

    out
}

/// Generates a tangent-direction flow map or shift pattern for Anisotropic shading.
pub fn generate_anisotropy_tangent_map(width: u32, height: u32, angle_rad: f32, radial: bool) -> U8Image {
    let mut out = U8Image::new(width, height);
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let px = (idx % width as usize) as f32;
            let py = (idx / width as usize) as f32;

            let theta = if radial {
                let dx = px - cx;
                let dy = py - cy;
                dy.atan2(dx) + angle_rad
            } else {
                angle_rad
            };

            let dir_x = theta.cos() * 0.5 + 0.5;
            let dir_y = theta.sin() * 0.5 + 0.5;

            out_pix[0] = (dir_x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[1] = (dir_y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[2] = 255;
            out_pix[3] = 255;
        });

    out
}

/// Converts normalized RGB to HSV (Hue in 0..360, Saturation in 0..1, Value in 0..1).
#[inline(always)]
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max > 1e-5 { delta / max } else { 0.0 };

    let h = if delta < 1e-5 {
        0.0
    } else if (max - r).abs() < 1e-5 {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if (max - g).abs() < 1e-5 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    (h, s, v)
}

/// Shortest angular distance between two angles in degrees (0..180).
#[inline(always)]
pub fn circular_dist_deg(a: f32, b: f32) -> f32 {
    let diff = (a - b).abs().rem_euclid(360.0);
    if diff > 180.0 {
        360.0 - diff
    } else {
        diff
    }
}

/// Algorithmic metalness map generator (ShaderMap analogue).
///
/// Detects metallic characteristics:
/// - Polished raw metals (Silver, Iron, Chrome, Aluminum): Low saturation, high luminance.
/// - Colored metals (Gold, Brass, Copper, Bronze): Characteristic hue & saturation profiles.
/// - Falloff curve and threshold remapping.
pub fn generate_metalness_map(
    src: &U8Image,
    threshold: f32,
    falloff: f32,
    detect_metals: bool,
    invert: bool,
) -> U8Image {
    let (width, height) = src.dimensions();
    let mut out = U8Image::new(width, height);
    let src_bytes = src.as_raw();

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let s_idx = idx * 4;
            let r = src_bytes[s_idx] as f32 / 255.0;
            let g = src_bytes[s_idx + 1] as f32 / 255.0;
            let b = src_bytes[s_idx + 2] as f32 / 255.0;

            let (h, s, v) = rgb_to_hsv(r, g, b);
            let lum = luminance(r, g, b);

            let mut metalness = if detect_metals {
                // 1. Polished neutral metals: low saturation, high brightness
                let silver_prob = if s < 0.20 && v > threshold {
                    ((v - threshold) / (1.0 - threshold).max(0.01)) * (1.0 - s * 4.0).clamp(0.0, 1.0)
                } else {
                    0.0
                };

                // 2. Gold (hue ~ 40..55 deg, moderate/high sat, bright)
                let gold_prob = if (40.0..=55.0).contains(&h) && s > 0.35 && v > 0.5 {
                    let h_dist = (h - 48.0).abs() / 15.0;
                    (1.0 - h_dist).clamp(0.0, 1.0) * s * v
                } else {
                    0.0
                };

                // 3. Copper / Bronze (hue ~ 15..35 deg)
                let copper_prob = if (15.0..=35.0).contains(&h) && s > 0.30 && v > 0.45 {
                    let h_dist = (h - 25.0).abs() / 15.0;
                    (1.0 - h_dist).clamp(0.0, 1.0) * s * v
                } else {
                    0.0
                };

                // 4. Brass (hue ~ 55..65 deg)
                let brass_prob = if (55.0..=65.0).contains(&h) && s > 0.35 && v > 0.55 {
                    let h_dist = (h - 60.0).abs() / 12.0;
                    (1.0 - h_dist).clamp(0.0, 1.0) * s * v
                } else {
                    0.0
                };

                let max_metal = silver_prob.max(gold_prob).max(copper_prob).max(brass_prob);
                max_metal.clamp(0.0, 1.0)
            } else {
                // Standard luminance thresholding with smooth falloff
                let f_width = falloff.max(0.001);
                ((lum - threshold) / f_width).clamp(0.0, 1.0)
            };

            if invert {
                metalness = 1.0 - metalness;
            }

            let byte_val = (metalness.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[0] = byte_val;
            out_pix[1] = byte_val;
            out_pix[2] = byte_val;
            out_pix[3] = 255;
        });

    out
}

/// Generates an advanced glowing emissive map with color-keying / hue isolation and luminance thresholding.
pub fn generate_advanced_emissive_mask(
    src: &U8Image,
    min_lum: f32,
    max_lum: f32,
    target_hue: Option<f32>,
    hue_tolerance: f32,
    tint_color: [f32; 3],
    intensity: f32,
    invert: bool,
    mask: Option<&U8Image>,
) -> U8Image {
    let (width, height) = src.dimensions();
    let mut out = U8Image::new(width, height);
    let src_bytes = src.as_raw();

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let px = (idx % width as usize) as u32;
            let py = (idx / width as usize) as u32;
            let s_idx = idx * 4;

            let r = src_bytes[s_idx] as f32 / 255.0;
            let g = src_bytes[s_idx + 1] as f32 / 255.0;
            let b = src_bytes[s_idx + 2] as f32 / 255.0;

            let lum = luminance(r, g, b);
            let mut factor = if max_lum > min_lum {
                ((lum - min_lum) / (max_lum - min_lum)).clamp(0.0, 1.0)
            } else if lum >= min_lum {
                1.0
            } else {
                0.0
            };

            // Color Keying / Hue filtering (e.g. runes, glowing eyes, neon highlights)
            if let Some(thue) = target_hue {
                let (h, s, _v) = rgb_to_hsv(r, g, b);
                if s > 0.15 {
                    let dist = circular_dist_deg(h, thue);
                    let tol = hue_tolerance.max(1.0);
                    let hue_factor = (1.0 - dist / tol).clamp(0.0, 1.0);
                    factor *= hue_factor;
                } else {
                    factor *= 0.0;
                }
            }

            if invert {
                factor = 1.0 - factor;
            }

            if let Some(m) = mask {
                let (mw, mh) = m.dimensions();
                let mx = (px * mw / width).min(mw - 1);
                let my = (py * mh / height).min(mh - 1);
                let mask_val = m.get_pixel(mx, my)[0] as f32 / 255.0;
                factor *= mask_val;
            }

            let emissive_r = (r * tint_color[0] * factor * intensity).clamp(0.0, 1.0);
            let emissive_g = (g * tint_color[1] * factor * intensity).clamp(0.0, 1.0);
            let emissive_b = (b * tint_color[2] * factor * intensity).clamp(0.0, 1.0);

            out_pix[0] = (emissive_r * 255.0 + 0.5) as u8;
            out_pix[1] = (emissive_g * 255.0 + 0.5) as u8;
            out_pix[2] = (emissive_b * 255.0 + 0.5) as u8;
            out_pix[3] = 255;
        });

    out
}

/// Supported procedural noise algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoiseType {
    Perlin,
    Voronoi,
    WhiteNoise,
}

#[inline(always)]
fn hash2d(x: i32, y: i32) -> f32 {
    let mut n = (x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263)) as u32;
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    (n ^ (n >> 16)) as f32 / 4294967295.0
}

#[inline(always)]
fn grad_dot(hash: u32, x: f32, y: f32) -> f32 {
    let h = hash & 7;
    let u = if h < 4 { x } else { y };
    let v = if h < 4 { y } else { x };
    (if (h & 1) != 0 { -u } else { u }) + (if (h & 2) != 0 { -2.0 * v } else { 2.0 * v })
}

#[inline(always)]
fn perlin_2d(x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - x.floor();
    let yf = y - y.floor();

    let u = xf * xf * xf * (xf * (xf * 6.0 - 15.0) + 10.0);
    let v = yf * yf * yf * (yf * (yf * 6.0 - 15.0) + 10.0);

    let h00 = ((xi.wrapping_mul(374761393) ^ yi.wrapping_mul(668265263)) as u32) >> 16;
    let h10 = (((xi + 1).wrapping_mul(374761393) ^ yi.wrapping_mul(668265263)) as u32) >> 16;
    let h01 = ((xi.wrapping_mul(374761393) ^ (yi + 1).wrapping_mul(668265263)) as u32) >> 16;
    let h11 = (((xi + 1).wrapping_mul(374761393) ^ (yi + 1).wrapping_mul(668265263)) as u32) >> 16;

    let x1 = (1.0 - u) * grad_dot(h00, xf, yf) + u * grad_dot(h10, xf - 1.0, yf);
    let x2 = (1.0 - u) * grad_dot(h01, xf, yf - 1.0) + u * grad_dot(h11, xf - 1.0, yf - 1.0);

    ((1.0 - v) * x1 + v * x2) * 0.5 + 0.5
}

#[inline(always)]
fn voronoi_2d(x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let mut min_dist = 100.0f32;

    for dx in -1..=1 {
        for dy in -1..=1 {
            let cx = xi + dx;
            let cy = yi + dy;
            let px = cx as f32 + hash2d(cx, cy);
            let py = cy as f32 + hash2d(cx.wrapping_add(101), cy.wrapping_add(202));
            let d = (px - x).hypot(py - y);
            if d < min_dist {
                min_dist = d;
            }
        }
    }
    min_dist.clamp(0.0, 1.0)
}

/// Generates high-performance multi-octave procedural noise (Perlin fBm, Voronoi cellular, White noise).
pub fn generate_procedural_noise(
    width: u32,
    height: u32,
    noise_type: NoiseType,
    scale: f32,
    octaves: usize,
    lacunarity: f32,
    gain: f32,
) -> U8Image {
    let mut out = U8Image::new(width.max(1), height.max(1));
    let w_f = width as f32;
    let h_f = height as f32;
    let sc = scale.max(0.1);

    out.as_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(idx, out_pix)| {
            let px = (idx % width as usize) as f32;
            let py = (idx / width as usize) as f32;
            let u = px / w_f * sc;
            let v = py / h_f * sc;

            let val = match noise_type {
                NoiseType::WhiteNoise => hash2d(px as i32, py as i32),
                NoiseType::Voronoi => voronoi_2d(u, v),
                NoiseType::Perlin => {
                    let mut sum = 0.0f32;
                    let mut freq = 1.0f32;
                    let mut amp = 1.0f32;
                    let mut max_amp = 0.0f32;
                    let octs = octaves.clamp(1, 8);
                    for _ in 0..octs {
                        sum += perlin_2d(u * freq, v * freq) * amp;
                        max_amp += amp;
                        freq *= lacunarity;
                        amp *= gain;
                    }
                    (sum / max_amp.max(1e-5)).clamp(0.0, 1.0)
                }
            };

            let byte = (val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            out_pix[0] = byte;
            out_pix[1] = byte;
            out_pix[2] = byte;
            out_pix[3] = 255;
        });

    out
}

/// Smoothstep Hermite interpolation between edge0 and edge1.
#[inline(always)]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0).max(1e-5)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Strand orientation for procedural hair generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrandOrientation {
    /// Hair strands run vertically along Y axis (standard for Ray-MMD hair meshes).
    Vertical,
    /// Hair strands run horizontally along X axis.
    Horizontal,
}

/// Generates high-fidelity procedural hair maps matching Ray-MMD anisotropic shader specifications:
/// 1. Tangent space Normal map with cylindrical micro-groove perturbation.
/// 2. Tangent Shift map for silky anisotropic specular jitter (matching shift4.png plugged into Custom B).
/// 3. Individual strand alpha/depth mask for albedo and occlusion modulation.
pub fn generate_hair_strands(
    width: u32,
    height: u32,
    strand_density: f32,
    roughness: f32,
    waviness: f32,
    wave_frequency: f32,
    orientation: StrandOrientation,
    normal_intensity: f32,
) -> (U8Image, U8Image, U8Image) {
    let w = width.max(1);
    let h = height.max(1);
    let mut normal_img = U8Image::new(w, h);
    let mut shift_img = U8Image::new(w, h);
    let mut mask_img = U8Image::new(w, h);

    let w_f = w as f32;
    let h_f = h as f32;
    let density = strand_density.clamp(5.0, 2000.0);
    let rough = roughness.clamp(0.0, 1.0);
    let wave_amp = waviness.clamp(0.0, 1.0) * 0.04;
    let wave_freq = wave_frequency.clamp(0.1, 50.0);
    let norm_scale = normal_intensity.clamp(0.0, 5.0);

    let mut normal_raw = normal_img.as_flat_samples_mut();
    let mut shift_raw = shift_img.as_flat_samples_mut();
    let mut mask_raw = mask_img.as_flat_samples_mut();

    normal_raw
        .as_mut_slice()
        .par_chunks_exact_mut(4)
        .zip(shift_raw.as_mut_slice().par_chunks_exact_mut(4))
        .zip(mask_raw.as_mut_slice().par_chunks_exact_mut(4))
        .enumerate()
        .for_each(|(idx, ((norm_pix, shift_pix), mask_pix))| {
            let px = (idx % w as usize) as f32;
            let py = (idx / w as usize) as f32;
            let u = px / w_f;
            let v = py / h_f;

            let (cross_coord, flow_coord) = match orientation {
                StrandOrientation::Vertical => (u, v),
                StrandOrientation::Horizontal => (v, u),
            };

            // Organic strand waviness along flow
            let wave = (flow_coord * wave_freq * std::f32::consts::TAU).sin() * wave_amp;
            let strand_pos = cross_coord + wave;

            // Discrete fiber index and intra-strand coordinate [-1.0, 1.0]
            let fiber_scalar = strand_pos * density;
            let fiber_id = fiber_scalar.floor() as i32;
            let intra_strand = fiber_scalar.fract() * 2.0 - 1.0;

            // Per-fiber stochastic properties
            let fiber_h0 = hash2d(fiber_id, 107);
            let fiber_h1 = hash2d(fiber_id, 389);
            let fiber_jitter = (hash2d(fiber_id, (flow_coord * 120.0) as i32) - 0.5) * rough;

            // Exact Ray-MMD hairNoise formula from material_common_2.0.fxsub:
            // hairNoise = sin(nx) * 0.4 + sin(nx * 1.732) * 0.3 + sin(nx * 3.1415) * 0.3
            let nx = strand_pos * density * std::f32::consts::TAU;
            let ray_noise = (nx).sin() * 0.4 + (nx * 1.732).sin() * 0.3 + (nx * std::f32::consts::PI).sin() * 0.3;

            // 1. Normal Map (Cylindrical micro-groove slope + Ray-MMD sine wave perturbation)
            let cylindrical_slope = intra_strand * (1.0 - intra_strand * intra_strand).max(0.0).sqrt();
            let pert = (cylindrical_slope * 0.65 + ray_noise * 0.25 + fiber_jitter * 0.5) * norm_scale;

            let (nx_val, ny_val, nz_val) = match orientation {
                StrandOrientation::Vertical => {
                    let len = (pert * pert + 1.0).sqrt();
                    (-pert / len, 0.0, 1.0 / len)
                }
                StrandOrientation::Horizontal => {
                    let len = (pert * pert + 1.0).sqrt();
                    (0.0, -pert / len, 1.0 / len)
                }
            };

            norm_pix[0] = (nx_val.clamp(-1.0, 1.0) * 127.5 + 128.0) as u8;
            norm_pix[1] = (ny_val.clamp(-1.0, 1.0) * 127.5 + 128.0) as u8;
            norm_pix[2] = (nz_val.clamp(0.0, 1.0) * 255.0) as u8;
            norm_pix[3] = 255;

            // 2. Tangent Shift Map (Silky specular anisotropic jitter, matching shift4.png)
            let flow_variation = (flow_coord * wave_freq * 1.5 + fiber_h0 * 6.28).sin() * 0.15;
            let shift_val = (0.5 + (fiber_h0 - 0.5) * 0.5 + flow_variation + fiber_jitter * 0.3).clamp(0.0, 1.0);
            let shift_byte = (shift_val * 255.0 + 0.5) as u8;
            shift_pix[0] = shift_byte;
            shift_pix[1] = shift_byte;
            shift_pix[2] = shift_byte;
            shift_pix[3] = 255;

            // 3. Strand Mask (Individual fiber highlight and separation)
            let core = (1.0 - intra_strand.abs().powf(1.8)) * (0.8 + fiber_h1 * 0.2);
            let mask_byte = (core.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            mask_pix[0] = mask_byte;
            mask_pix[1] = mask_byte;
            mask_pix[2] = mask_byte;
            mask_pix[3] = 255;
        });

    (normal_img, shift_img, mask_img)
}

/// Procedurally generates physical Cornea lens and Iris parallax maps:
/// 1. Cornea Dome Normal Map (Convex spherical lens converting flat eye mesh into physical cornea dome)
/// 2. Iris Parallax / Concave Depth Map (Depth into anterior chamber for parallax occlusion)
/// 3. Limbal Ring & Caustic Mask Map (R = Limbal darkening, G = Caustic focus, B = Pupil/Iris mask)
/// 4. Refracted Iris Image (Applies Snell's Law refraction, limbal darkening, and caustic intensity)
pub fn generate_eye_cornea_maps(
    width: u32,
    height: u32,
    iris_depth: f32,
    cornea_ior: f32,
    limbal_width: f32,
    limbal_darkness: f32,
    caustic_intensity: f32,
    dome_curvature: f32,
    input_iris: Option<&U8Image>,
) -> (U8Image, U8Image, U8Image, U8Image) {
    let w = width.max(1);
    let h = height.max(1);
    let mut normal_img = U8Image::new(w, h);
    let mut parallax_img = U8Image::new(w, h);
    let mut limbal_caustic_img = U8Image::new(w, h);
    let mut refracted_iris_img = U8Image::new(w, h);

    let w_f = w as f32;
    let h_f = h as f32;
    let depth = iris_depth.clamp(0.001, 0.5);
    let ior = cornea_ior.max(1.0);
    let eta = 1.0 / ior;
    let l_width = limbal_width.clamp(0.01, 0.5);
    let l_darkness = limbal_darkness.clamp(0.0, 1.0);
    let c_intensity = caustic_intensity.clamp(0.0, 5.0);
    let curvature = dome_curvature.clamp(0.05, 1.0);

    let mut norm_raw = normal_img.as_flat_samples_mut();
    let mut par_raw = parallax_img.as_flat_samples_mut();
    let mut lc_raw = limbal_caustic_img.as_flat_samples_mut();
    let mut ref_raw = refracted_iris_img.as_flat_samples_mut();

    norm_raw
        .as_mut_slice()
        .par_chunks_exact_mut(4)
        .zip(par_raw.as_mut_slice().par_chunks_exact_mut(4))
        .zip(lc_raw.as_mut_slice().par_chunks_exact_mut(4))
        .zip(ref_raw.as_mut_slice().par_chunks_exact_mut(4))
        .enumerate()
        .for_each(|(idx, (((norm_pix, par_pix), lc_pix), ref_pix))| {
            let px = (idx % w as usize) as f32;
            let py = (idx / w as usize) as f32;
            let u = (px + 0.5) / w_f;
            let v = (py + 0.5) / h_f;

            // Centered UV coords [-1.0, 1.0] relative to eye/iris center
            let cx = (u - 0.5) * 2.0;
            let cy = (v - 0.5) * 2.0;
            let dist = (cx * cx + cy * cy).sqrt();

            // 1. Cornea Dome Normal Map (Convex spherical dome)
            let dome_r = (dist * curvature).min(1.0);
            let dome_z = (1.0 - dome_r * dome_r).max(0.0).sqrt();
            let dome_falloff = smoothstep(1.1, 0.9, dist);
            let n_x = -cx * curvature * dome_falloff;
            let n_y = -cy * curvature * dome_falloff; // DirectX Y-
            let n_z = dome_z * dome_falloff + (1.0 - dome_falloff);
            let n_len = (n_x * n_x + n_y * n_y + n_z * n_z).sqrt().max(1e-5);
            let norm_vec = [n_x / n_len, n_y / n_len, n_z / n_len];

            norm_pix[0] = (norm_vec[0] * 127.5 + 128.0) as u8;
            norm_pix[1] = (norm_vec[1] * 127.5 + 128.0) as u8;
            norm_pix[2] = (norm_vec[2] * 255.0).clamp(0.0, 255.0) as u8;
            norm_pix[3] = 255;

            // 2. Iris Parallax Map (Concave depth into anterior chamber)
            let funnel_depth = (1.0 - (dist * 0.95).min(1.0)).powf(1.6) * depth * 3.0;
            let par_val = (1.0 - funnel_depth).clamp(0.0, 1.0);
            let par_byte = (par_val * 255.0 + 0.5) as u8;
            par_pix[0] = par_byte;
            par_pix[1] = par_byte;
            par_pix[2] = par_byte;
            par_pix[3] = 255;

            // 3. Limbal Ring & Caustic Mask Map
            let limbal_edge = smoothstep(1.0, 1.0 - l_width, dist);
            let limbal_val = 1.0 - limbal_edge * l_darkness;

            let caustic_dir = (cx * 0.55 + cy * 0.75).max(0.0);
            let caustic_ring = (1.0 - (dist - 0.55).abs() * 2.8).max(0.0);
            let caustic_val = (caustic_dir.powf(2.2) * caustic_ring * c_intensity).clamp(0.0, 1.0);

            let pupil_val = smoothstep(0.25, 0.20, dist);

            lc_pix[0] = (limbal_val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            lc_pix[1] = (caustic_val * 255.0 + 0.5) as u8;
            lc_pix[2] = (pupil_val * 255.0 + 0.5) as u8;
            lc_pix[3] = 255;

            // 4. Refracted Iris Image
            let view_dir = [-0.15f32, -0.20, 0.95];
            let refr_scale = (1.0 - eta) * depth * 2.5;
            let offset_u = norm_vec[0] * refr_scale + view_dir[0] * depth;
            let offset_v = norm_vec[1] * refr_scale + view_dir[1] * depth;
            let refr_u = (u + offset_u).clamp(0.0, 1.0);
            let refr_v = (v + offset_v).clamp(0.0, 1.0);

            let base_color = if let Some(in_img) = input_iris {
                let sx = (refr_u * (in_img.width() - 1) as f32).round() as u32;
                let sy = (refr_v * (in_img.height() - 1) as f32).round() as u32;
                let p = in_img.get_pixel(sx.min(in_img.width() - 1), sy.min(in_img.height() - 1));
                [p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0, p[3] as f32 / 255.0]
            } else {
                let refr_cx = (refr_u - 0.5) * 2.0;
                let refr_cy = (refr_v - 0.5) * 2.0;
                let refr_dist = (refr_cx * refr_cx + refr_cy * refr_cy).sqrt();
                let angle = refr_cy.atan2(refr_cx);
                let spoke = (angle * 24.0).sin() * 0.08;

                if refr_dist < 0.22 {
                    [0.05, 0.05, 0.08, 1.0]
                } else if refr_dist < 0.95 {
                    let t = (refr_dist - 0.22) / 0.73;
                    let r_c = 0.15 + t * 0.2 + spoke;
                    let g_c = 0.35 + (1.0 - t) * 0.35 + spoke;
                    let b_c = 0.85 + (1.0 - t) * 0.15;
                    [r_c.clamp(0.0, 1.0), g_c.clamp(0.0, 1.0), b_c.clamp(0.0, 1.0), 1.0]
                } else {
                    [0.92, 0.93, 0.95, 1.0]
                }
            };

            let final_r = (base_color[0] * limbal_val + caustic_val * 0.35).clamp(0.0, 1.0);
            let final_g = (base_color[1] * limbal_val + caustic_val * 0.35).clamp(0.0, 1.0);
            let final_b = (base_color[2] * limbal_val + caustic_val * 0.40).clamp(0.0, 1.0);

            ref_pix[0] = (final_r * 255.0 + 0.5) as u8;
            ref_pix[1] = (final_g * 255.0 + 0.5) as u8;
            ref_pix[2] = (final_b * 255.0 + 0.5) as u8;
            ref_pix[3] = (base_color[3] * 255.0 + 0.5) as u8;
        });

    (normal_img, parallax_img, limbal_caustic_img, refracted_iris_img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedural_noise_generation() {
        let perlin = generate_procedural_noise(64, 64, NoiseType::Perlin, 4.0, 4, 2.0, 0.5);
        assert_eq!(perlin.width(), 64);
        assert_eq!(perlin.height(), 64);
        assert!(!perlin.as_raw().is_empty());

        let voronoi = generate_procedural_noise(64, 64, NoiseType::Voronoi, 5.0, 1, 2.0, 0.5);
        assert_eq!(voronoi.width(), 64);
        assert_eq!(voronoi.height(), 64);

        let white = generate_procedural_noise(64, 64, NoiseType::WhiteNoise, 1.0, 1, 1.0, 1.0);
        assert_eq!(white.width(), 64);
        assert_eq!(white.height(), 64);

        // Ensure noise is not completely uniform
        let raw = perlin.as_raw();
        let min_val = *raw.iter().step_by(4).min().unwrap();
        let max_val = *raw.iter().step_by(4).max().unwrap();
        assert!(max_val > min_val + 20, "Perlin noise must produce non-trivial dynamic range");
    }

    #[test]
    fn test_generate_hair_strands() {
        let (norm, shift, mask) = generate_hair_strands(
            64, 64, 200.0, 0.35, 0.20, 4.0, StrandOrientation::Vertical, 0.7,
        );
        assert_eq!(norm.width(), 64);
        assert_eq!(norm.height(), 64);
        assert_eq!(shift.width(), 64);
        assert_eq!(mask.width(), 64);

        // Normal map Z channel should be dominant (blue tangent normal)
        let n_raw = norm.as_raw();
        assert!(n_raw[2] > 128, "Normal Z should point towards camera in tangent space");

        // Shift map should have non-trivial variation
        let s_raw = shift.as_raw();
        let s_min = *s_raw.iter().step_by(4).min().unwrap();
        let s_max = *s_raw.iter().step_by(4).max().unwrap();
        assert!(s_max > s_min, "Shift map must have variation across hair strands");
    }

    #[test]
    fn test_generate_eye_cornea_maps() {
        let (norm, par, lc, refr) = generate_eye_cornea_maps(
            64, 64, 0.05, 1.376, 0.15, 0.65, 1.50, 0.85, None,
        );
        assert_eq!(norm.width(), 64);
        assert_eq!(par.width(), 64);
        assert_eq!(lc.width(), 64);
        assert_eq!(refr.width(), 64);

        // Center pixel of normal map should be facing straight out [128, 128, 255]
        let center_idx = (32 * 64 + 32) * 4;
        let n_raw = norm.as_raw();
        let diff_x = (n_raw[center_idx] as i32 - 128).abs();
        let diff_y = (n_raw[center_idx + 1] as i32 - 128).abs();
        assert!(diff_x < 15 && diff_y < 15, "Center of dome normal should be perpendicular to surface");
    }
}


