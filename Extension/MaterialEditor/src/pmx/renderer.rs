//! High-performance software PBR rasterizer for PMX 3D models with multi-materials.
//!
//! Features:
//! - Multi-threaded band/tile rasterization with perspective-correct Z-buffer
//! - Per-subset material shading with Cook-Torrance GGX PBR
//! - Live support for Custom A & Custom B shading models (ClearCoat, Cloth, Skin/SSS, Cel Shading)
//! - Subsets isolation (Solo) and active subset selection highlighting
//! - Texture sampling with bilinear filtering and mip/wrap support

use glam::{Mat4, Vec2, Vec3, Vec4};
use rayon::prelude::*;
use crate::graph::eval::EvaluatedMaterial;
use crate::graph::node::ShadingModel;
use crate::image_proc::U8Image;
use crate::pmx::{PmxModel, PmxSubset};
use crate::viewport::{PreviewCamera, ViewportDisplayMode};


/// Helper to sample an image with bilinear filtering and texture wrap/clamp.
#[inline(always)]
fn sample_texture(img: Option<&U8Image>, uv: Vec2, default: [f32; 4]) -> [f32; 4] {
    let img = match img {
        Some(m) if m.width() > 0 && m.height() > 0 => m,
        _ => return default,
    };

    let w = img.width() as f32;
    let h = img.height() as f32;

    let u_wrapped = uv.x.rem_euclid(1.0);
    let v_wrapped = uv.y.rem_euclid(1.0);

    let fx = (u_wrapped * w - 0.5).max(0.0);
    let fy = (v_wrapped * h - 0.5).max(0.0);

    let x0 = (fx.floor() as usize).min(img.width() as usize - 1);
    let y0 = (fy.floor() as usize).min(img.height() as usize - 1);
    let x1 = (x0 + 1).min(img.width() as usize - 1);
    let y1 = (y0 + 1).min(img.height() as usize - 1);

    let wx = fx.fract();
    let wy = fy.fract();

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

/// Clip-space vertex for near plane clipping.
#[derive(Clone, Copy)]
struct ClipVertex {
    clip: Vec4,
    world_pos: Vec3,
    normal: Vec3,
    uv: Vec2,
}

/// Projected screen-space vertex.
#[derive(Clone, Copy)]
struct ScreenVertex {
    screen_pos: Vec2,
    inv_w: f32,
    depth: f32,
    world_pos: Vec3,
    normal: Vec3,
    uv: Vec2,
}

#[inline(always)]
fn project_to_screen(v: &ClipVertex, w_f: f32, h_f: f32) -> ScreenVertex {
    let inv_w = 1.0 / v.clip.w;
    let ndc_x = v.clip.x * inv_w;
    let ndc_y = v.clip.y * inv_w;
    let ndc_z = (v.clip.z * inv_w).clamp(0.0, 1.0);

    let sx = (ndc_x * 0.5 + 0.5) * w_f;
    let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * h_f;

    ScreenVertex {
        screen_pos: Vec2::new(sx, sy),
        inv_w,
        depth: ndc_z,
        world_pos: v.world_pos,
        normal: v.normal,
        uv: v.uv,
    }
}

/// Guard band multiplier for frustum side-plane clipping.
/// With G=4, NDC coords after clipping are bounded to [-4, 4], giving screen coords
/// roughly within [-1.5, 2.5] × viewport — well within f32 precision for barycentrics.
const FRUSTUM_GUARD_BAND: f32 = 4.0;

/// Maximum polygon vertices after clipping a triangle against 5 planes.
/// Each plane adds at most 1 vertex: 3 + 5 = 8. Padded to 10 for safety.
const MAX_CLIP_VERTS: usize = 10;

/// Linearly interpolates all ClipVertex attributes between two vertices.
#[inline(always)]
fn lerp_clip_vertex(a: &ClipVertex, b: &ClipVertex, t: f32) -> ClipVertex {
    ClipVertex {
        clip: a.clip.lerp(b.clip, t),
        world_pos: a.world_pos.lerp(b.world_pos, t),
        normal: a.normal.lerp(b.normal, t).normalize_or_zero(),
        uv: a.uv.lerp(b.uv, t),
    }
}

/// Sutherland-Hodgman clip of a convex polygon against a single half-plane.
/// `dists` contains pre-computed signed distances for each input vertex (>= 0 = inside).
/// Returns the number of output vertices written to `out`.
#[inline(always)]
fn sh_clip_plane(
    input: &[ClipVertex; MAX_CLIP_VERTS],
    in_count: usize,
    out: &mut [ClipVertex; MAX_CLIP_VERTS],
    dists: &[f32; MAX_CLIP_VERTS],
) -> usize {
    if in_count == 0 {
        return 0;
    }
    let mut n = 0usize;
    let mut prev = in_count - 1;
    for curr in 0..in_count {
        let dp = dists[prev];
        let dc = dists[curr];
        if dp >= 0.0 {
            if dc >= 0.0 {
                // Both inside — emit current
                out[n] = input[curr];
                n += 1;
            } else {
                // Leaving — emit intersection only
                let t = dp / (dp - dc);
                out[n] = lerp_clip_vertex(&input[prev], &input[curr], t);
                n += 1;
            }
        } else if dc >= 0.0 {
            // Entering — emit intersection then current
            let t = dp / (dp - dc);
            out[n] = lerp_clip_vertex(&input[prev], &input[curr], t);
            n += 1;
            if n < MAX_CLIP_VERTS {
                out[n] = input[curr];
                n += 1;
            }
        }
        prev = curr;
    }
    n
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn rasterize_screen_triangle(
    v0: &ScreenVertex,
    v1: &ScreenVertex,
    v2: &ScreenVertex,
    y_start: usize,
    y_end: usize,
    width: usize,
    z_buf: &mut [f32],
    pix_buf: &mut [u8],
    cam_pos: Vec3,
    light_dir: Vec3,
    light_color: Vec3,
    ambient_sky: Vec3,
    ambient_ground: Vec3,
    mat_eval: Option<&EvaluatedMaterial>,
    fallback_tex: Option<&U8Image>,
    subset: &PmxSubset,
    shading_model: ShadingModel,
    display_mode: ViewportDisplayMode,
    is_active: bool,
) {
    let edge0 = v1.screen_pos - v0.screen_pos;
    let edge1 = v2.screen_pos - v0.screen_pos;
    let det = edge0.x * edge1.y - edge0.y * edge1.x;
    if det.abs() < 1e-5 {
        return;
    }

    let min_x = v0.screen_pos.x.min(v1.screen_pos.x).min(v2.screen_pos.x).floor() as i32;
    let max_x = v0.screen_pos.x.max(v1.screen_pos.x).max(v2.screen_pos.x).ceil() as i32;
    let min_y = v0.screen_pos.y.min(v1.screen_pos.y).min(v2.screen_pos.y).floor() as i32;
    let max_y = v0.screen_pos.y.max(v1.screen_pos.y).max(v2.screen_pos.y).ceil() as i32;

    let start_x = min_x.max(0).min(width as i32) as usize;
    let end_x = max_x.max(0).min(width as i32) as usize;
    let start_y = min_y.max(y_start as i32).min(y_end as i32) as usize;
    let end_y = max_y.max(y_start as i32).min(y_end as i32) as usize;

    if start_x >= end_x || start_y >= end_y {
        return;
    }

    let inv_det = 1.0 / det;

    for y in start_y..end_y {
        let y_rel = y - y_start;
        let py = y as f32 + 0.5;

        for x in start_x..end_x {
            let px = x as f32 + 0.5;
            let p = Vec2::new(px, py);

            let w0 = ((v1.screen_pos.x - p.x) * (v2.screen_pos.y - p.y) - (v1.screen_pos.y - p.y) * (v2.screen_pos.x - p.x)) * inv_det;
            let w1 = ((v2.screen_pos.x - p.x) * (v0.screen_pos.y - p.y) - (v2.screen_pos.y - p.y) * (v0.screen_pos.x - p.x)) * inv_det;
            let w2 = 1.0 - w0 - w1;

            if w0 >= -1e-4 && w1 >= -1e-4 && w2 >= -1e-4 {
                let inv_w = w0 * v0.inv_w + w1 * v1.inv_w + w2 * v2.inv_w;
                if inv_w <= 0.0 {
                    continue;
                }
                let w_persp = 1.0 / inv_w;
                // ndc_z is already perspective-divided (clip.z / clip.w) and is strictly linear in screen space.
                // Do NOT multiply by inv_w and w_persp again, which causes extreme non-linear depth distortion!
                let depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;

                let pix_idx = y_rel * width + x;
                if depth < z_buf[pix_idx] {
                    z_buf[pix_idx] = depth;



                    let uv = (w0 * v0.uv * v0.inv_w + w1 * v1.uv * v1.inv_w + w2 * v2.uv * v2.inv_w) * w_persp;
                    let normal = (w0 * v0.normal * v0.inv_w + w1 * v1.normal * v1.inv_w + w2 * v2.normal * v2.inv_w) * w_persp;
                    let normal = normal.normalize_or_zero();
                    let world_pos = (w0 * v0.world_pos * v0.inv_w + w1 * v1.world_pos * v1.inv_w + w2 * v2.world_pos * v2.inv_w) * w_persp;

                    let final_rgba = shade_pmx_pixel(
                        world_pos,
                        normal,
                        uv,
                        cam_pos,
                        light_dir,
                        light_color,
                        ambient_sky,
                        ambient_ground,
                        mat_eval,
                        fallback_tex,
                        subset,
                        shading_model,
                        display_mode,
                        is_active,
                    );

                    let out_byte_idx = pix_idx * 4;
                    pix_buf[out_byte_idx] = final_rgba[0];
                    pix_buf[out_byte_idx + 1] = final_rgba[1];
                    pix_buf[out_byte_idx + 2] = final_rgba[2];
                    pix_buf[out_byte_idx + 3] = final_rgba[3];
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn clip_and_rasterize_triangle(
    cv0: &ClipVertex,
    cv1: &ClipVertex,
    cv2: &ClipVertex,
    w_f: f32,
    h_f: f32,
    y_start: usize,
    y_end: usize,
    width: usize,
    z_buf: &mut [f32],
    pix_buf: &mut [u8],
    cam_pos: Vec3,
    light_dir: Vec3,
    light_color: Vec3,
    ambient_sky: Vec3,
    ambient_ground: Vec3,
    mat_eval: Option<&EvaluatedMaterial>,
    fallback_tex: Option<&U8Image>,
    subset: &PmxSubset,
    shading_model: ShadingModel,
    display_mode: ViewportDisplayMode,
    is_active: bool,
) {
    let zero_cv = ClipVertex {
        clip: Vec4::ZERO, world_pos: Vec3::ZERO, normal: Vec3::ZERO, uv: Vec2::ZERO,
    };
    let mut buf_a = [zero_cv; MAX_CLIP_VERTS];
    let mut buf_b = [zero_cv; MAX_CLIP_VERTS];
    let mut dists = [0.0f32; MAX_CLIP_VERTS];

    buf_a[0] = *cv0;
    buf_a[1] = *cv1;
    buf_a[2] = *cv2;
    let mut count: usize = 3;

    // 1) Near plane: clip.z >= 0
    for i in 0..count { dists[i] = buf_a[i].clip.z; }
    count = sh_clip_plane(&buf_a, count, &mut buf_b, &dists);
    if count < 3 { return; }

    // 2) Left plane: x + G*w >= 0
    let g = FRUSTUM_GUARD_BAND;
    for i in 0..count { dists[i] = buf_b[i].clip.x + g * buf_b[i].clip.w; }
    count = sh_clip_plane(&buf_b, count, &mut buf_a, &dists);
    if count < 3 { return; }

    // 3) Right plane: G*w - x >= 0
    for i in 0..count { dists[i] = g * buf_a[i].clip.w - buf_a[i].clip.x; }
    count = sh_clip_plane(&buf_a, count, &mut buf_b, &dists);
    if count < 3 { return; }

    // 4) Bottom plane: y + G*w >= 0
    for i in 0..count { dists[i] = buf_b[i].clip.y + g * buf_b[i].clip.w; }
    count = sh_clip_plane(&buf_b, count, &mut buf_a, &dists);
    if count < 3 { return; }

    // 5) Top plane: G*w - y >= 0
    for i in 0..count { dists[i] = g * buf_a[i].clip.w - buf_a[i].clip.y; }
    count = sh_clip_plane(&buf_a, count, &mut buf_b, &dists);
    if count < 3 { return; }

    // Fan-triangulate the clipped polygon and rasterize each sub-triangle
    let sv0 = project_to_screen(&buf_b[0], w_f, h_f);
    for i in 1..count - 1 {
        let sv1 = project_to_screen(&buf_b[i], w_f, h_f);
        let sv2 = project_to_screen(&buf_b[i + 1], w_f, h_f);
        rasterize_screen_triangle(
            &sv0, &sv1, &sv2,
            y_start, y_end, width,
            z_buf, pix_buf,
            cam_pos, light_dir, light_color, ambient_sky, ambient_ground,
            mat_eval, fallback_tex, subset, shading_model, display_mode, is_active,
        );
    }
}

/// Renders the entire PMX model into an RGBA pixel buffer.
#[allow(deprecated)]
pub fn render_pmx_model(
    model: &PmxModel,
    subset_materials: &[Option<EvaluatedMaterial>],
    fallback_textures: &[Option<U8Image>],
    camera: &PreviewCamera,
    shading_model: ShadingModel,
    display_mode: ViewportDisplayMode,
    active_subset_idx: Option<usize>,
    solo_active_subset: bool,
    width: u32,
    height: u32,
) -> U8Image {
    let mut out_img = U8Image::new(width, height);
    if model.vertices.is_empty() || model.indices.is_empty() {
        return out_img;
    }

    let w_f = width as f32;
    let h_f = height as f32;
    let aspect = w_f / h_f;

    let target = camera.target;
    let dist = camera.distance.max(0.05);
    let cam_x = target.x + dist * camera.pitch.cos() * camera.yaw.sin();
    let cam_y = target.y + dist * camera.pitch.sin();
    let cam_z = target.z - dist * camera.pitch.cos() * camera.yaw.cos();
    let cam_pos = Vec3::new(cam_x, cam_y, cam_z);

    let view = Mat4::look_at_lh(cam_pos, target, Vec3::Y);
    let fov = 45.0f32.to_radians();
    let near = 0.2f32;
    let far = (model.radius * 30.0).max(100000.0);
    let proj = Mat4::perspective_lh(fov, aspect, near, far);
    let view_proj = proj * view;

    // Light setup
    let light_dir = Vec3::new(
        camera.light_pitch.cos() * camera.light_yaw.sin(),
        camera.light_pitch.sin(),
        camera.light_pitch.cos() * camera.light_yaw.cos(),
    )
    .normalize();
    let light_color = Vec3::ONE * camera.light_intensity;
    let ambient_sky = Vec3::new(0.2, 0.25, 0.35) * 0.45;
    let ambient_ground = Vec3::new(0.12, 0.1, 0.08) * 0.45;

    // Step 1: Compute clip-space coordinates for all vertices in parallel
    let clip_vertices: Vec<ClipVertex> = model
        .vertices
        .par_iter()
        .map(|v| {
            let p4 = Vec4::new(v.position.x, v.position.y, v.position.z, 1.0);
            let clip = view_proj * p4;
            ClipVertex {
                clip,
                world_pos: v.position,
                normal: v.normal.normalize_or_zero(),
                uv: v.uv,
            }
        })
        .collect();

    // Fast-path screen projection for vertices in front of near plane (clip.z >= 0 and clip.w > 0)
    let screen_vertices: Vec<Option<ScreenVertex>> = clip_vertices
        .par_iter()
        .map(|cv| {
            if cv.clip.z < 0.0 || cv.clip.w <= 0.0 {
                return None;
            }
            // Guard band: if NDC coords exceed the frustum guard band, route to
            // full clip path to prevent f32 precision loss in barycentric computation
            let inv_w = 1.0 / cv.clip.w;
            let ndc_x = cv.clip.x * inv_w;
            let ndc_y = cv.clip.y * inv_w;
            if ndc_x.abs() > FRUSTUM_GUARD_BAND || ndc_y.abs() > FRUSTUM_GUARD_BAND {
                return None;
            }
            Some(project_to_screen(cv, w_f, h_f))
        })
        .collect();

    // Step 2: Divide screen into horizontal bands and rasterize in parallel
    let band_count = 16.min(height as usize);
    let band_height = (height as usize + band_count - 1) / band_count;

    let bands: Vec<(usize, usize)> = (0..band_count)
        .map(|b| {
            let y_start = (b * band_height).min(height as usize);
            let y_end = (y_start + band_height).min(height as usize);
            (y_start, y_end)
        })
        .collect();

    // Output raster bands
    let rendered_bands: Vec<(usize, usize, Vec<u8>)> = bands
        .into_par_iter()
        .map(|(y_start, y_end)| {
            let b_height = y_end - y_start;
            let pixel_count = width as usize * b_height;
            let mut z_buf = vec![f32::MAX; pixel_count];
            let mut pix_buf = vec![0u8; pixel_count * 4];

            // Fill background gradient for this band
            for y_rel in 0..b_height {
                let py = y_start + y_rel;
                let grad = (py as f32 / h_f).clamp(0.0, 1.0);
                let bg_val = (0.12 - grad * 0.05).clamp(0.0, 1.0);
                let bg_byte = (bg_val * 255.0 + 0.5) as u8;

                for x in 0..width as usize {
                    let idx = (y_rel * width as usize + x) * 4;
                    pix_buf[idx] = bg_byte;
                    pix_buf[idx + 1] = (bg_byte as f32 * 1.05).min(255.0) as u8;
                    pix_buf[idx + 2] = (bg_byte as f32 * 1.15).min(255.0) as u8;
                    pix_buf[idx + 3] = 255;
                }
            }

            // Iterate over all subsets
            for (sub_idx, subset) in model.subsets.iter().enumerate() {
                if !subset.is_visible {
                    continue;
                }

                let is_active = active_subset_idx == Some(sub_idx);
                if solo_active_subset && !is_active {
                    continue;
                }

                let mat_eval = subset_materials.get(sub_idx).and_then(|m| m.as_ref());
                let fallback_tex = fallback_textures.get(sub_idx).and_then(|t| t.as_ref());

                let sub_end = subset.index_start + subset.index_count;
                let tri_count = subset.index_count / 3;

                for t in 0..tri_count {
                    let base = subset.index_start + t * 3;
                    if base + 2 >= sub_end || base + 2 >= model.indices.len() {
                        break;
                    }

                    let i0 = model.indices[base] as usize;
                    let i1 = model.indices[base + 1] as usize;
                    let i2 = model.indices[base + 2] as usize;

                    if i0 >= clip_vertices.len() || i1 >= clip_vertices.len() || i2 >= clip_vertices.len() {
                        continue;
                    }

                    match (
                        screen_vertices.get(i0).copied().flatten(),
                        screen_vertices.get(i1).copied().flatten(),
                        screen_vertices.get(i2).copied().flatten(),
                    ) {
                        (Some(sv0), Some(sv1), Some(sv2)) => {
                            // Fast path: All 3 vertices in front of near plane
                            rasterize_screen_triangle(
                                &sv0,
                                &sv1,
                                &sv2,
                                y_start,
                                y_end,
                                width as usize,
                                &mut z_buf,
                                &mut pix_buf,
                                cam_pos,
                                light_dir,
                                light_color,
                                ambient_sky,
                                ambient_ground,
                                mat_eval,
                                fallback_tex,
                                subset,
                                shading_model,
                                display_mode,
                                is_active,
                            );
                        }
                        _ => {
                            // Near-plane clipping path: at least one vertex is behind or crossing near plane
                            let cv0 = &clip_vertices[i0];
                            let cv1 = &clip_vertices[i1];
                            let cv2 = &clip_vertices[i2];
                            clip_and_rasterize_triangle(
                                cv0,
                                cv1,
                                cv2,
                                w_f,
                                h_f,
                                y_start,
                                y_end,
                                width as usize,
                                &mut z_buf,
                                &mut pix_buf,
                                cam_pos,
                                light_dir,
                                light_color,
                                ambient_sky,
                                ambient_ground,
                                mat_eval,
                                fallback_tex,
                                subset,
                                shading_model,
                                display_mode,
                                is_active,
                            );
                        }
                    }
                }
            }

            (y_start, y_end, pix_buf)
        })
        .collect();

    // Step 3: Copy rendered bands to output image
    let mut flat = out_img.as_flat_samples_mut();
    let out_slice = flat.as_mut_slice();
    for (y_start, y_end, band_bytes) in rendered_bands {
        let b_height = y_end - y_start;
        let start_idx = y_start * width as usize * 4;
        let len = width as usize * b_height * 4;
        out_slice[start_idx..start_idx + len].copy_from_slice(&band_bytes);
    }

    out_img
}

/// Computes the final RGBA color for a PMX pixel with Cook-Torrance GGX and Custom A/B maps.
#[inline(always)]
fn shade_pmx_pixel(
    world_pos: Vec3,
    geo_normal: Vec3,
    uv: Vec2,
    cam_pos: Vec3,
    light_dir: Vec3,
    light_color: Vec3,
    ambient_sky: Vec3,
    ambient_ground: Vec3,
    mat_eval: Option<&EvaluatedMaterial>,
    fallback_tex: Option<&U8Image>,
    subset: &PmxSubset,
    shading_model: ShadingModel,
    display_mode: ViewportDisplayMode,
    is_active_subset: bool,
) -> [u8; 4] {
    let view_dir = (cam_pos - world_pos).normalize();

    // 1. Albedo
    let default_diffuse = [subset.diffuse[0], subset.diffuse[1], subset.diffuse[2], subset.diffuse[3]];
    let albedo_samp = if let Some(m) = mat_eval {
        if let Some(ref alb) = m.albedo {
            sample_texture(Some(alb), uv, default_diffuse)
        } else {
            sample_texture(fallback_tex, uv, default_diffuse)
        }
    } else {
        sample_texture(fallback_tex, uv, default_diffuse)
    };
    let albedo = Vec3::new(albedo_samp[0], albedo_samp[1], albedo_samp[2]);

    // 2. Normal
    let normal = if let Some(m) = mat_eval {
        if let Some(ref n_map) = m.normal {
            let n_samp = sample_texture(Some(n_map), uv, [0.5, 0.5, 1.0, 1.0]);
            let map_n = Vec3::new(n_samp[0] * 2.0 - 1.0, n_samp[1] * 2.0 - 1.0, n_samp[2] * 2.0 - 1.0).normalize();
            // Perturb normal relative to geometric normal
            (geo_normal + map_n * 0.7).normalize()
        } else {
            geo_normal
        }
    } else {
        geo_normal
    };

    // Two-sided normal check: ensures backfaces and thin geometry (hair, cloth, skirts) are lit correctly
    let normal = if normal.dot(view_dir) < 0.0 {
        -normal
    } else {
        normal
    };

    // 3. Roughness & Metalness
    let rough_samp = mat_eval
        .and_then(|m| m.smoothness.as_ref())
        .map(|s| sample_texture(Some(s), uv, [0.5, 0.5, 0.5, 1.0])[0])
        .unwrap_or(0.5);
    let roughness = (1.0 - rough_samp).clamp(0.04, 1.0);
    let alpha = roughness * roughness;

    let metal_samp = mat_eval
        .and_then(|m| m.metalness.as_ref())
        .map(|s| sample_texture(Some(s), uv, [0.0, 0.0, 0.0, 1.0])[0])
        .unwrap_or(0.0);
    let metalness = metal_samp.clamp(0.0, 1.0);

    // 4. Occlusion & Emissive
    let ao_samp = mat_eval
        .and_then(|m| m.occlusion.as_ref())
        .map(|s| sample_texture(Some(s), uv, [1.0, 1.0, 1.0, 1.0])[0])
        .unwrap_or(1.0);
    let ao = ao_samp.clamp(0.0, 1.0);

    let emissive_samp = mat_eval
        .and_then(|m| m.emissive.as_ref())
        .map(|s| sample_texture(Some(s), uv, [0.0, 0.0, 0.0, 1.0]))
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let emissive = Vec3::new(emissive_samp[0], emissive_samp[1], emissive_samp[2]);

    // 5. Custom A & Custom B
    let custom_a_samp = mat_eval
        .and_then(|m| m.custom_a.as_ref())
        .map(|s| sample_texture(Some(s), uv, [0.5, 0.5, 0.5, 1.0]))
        .unwrap_or([0.5, 0.5, 0.5, 1.0]);
    let custom_a = custom_a_samp[0];

    let custom_b_samp = mat_eval
        .and_then(|m| m.custom_b.as_ref())
        .map(|s| sample_texture(Some(s), uv, [1.0, 1.0, 1.0, 1.0]))
        .unwrap_or([1.0, 1.0, 1.0, 1.0]);
    let custom_b = Vec3::new(custom_b_samp[0], custom_b_samp[1], custom_b_samp[2]);

    // Check display modes
    match display_mode {
        ViewportDisplayMode::AlbedoOnly => {
            return to_u8_rgba(albedo);
        }
        ViewportDisplayMode::NormalOnly => {
            return to_u8_rgba(normal * 0.5 + Vec3::splat(0.5));
        }
        ViewportDisplayMode::RoughnessOnly => {
            return to_u8_rgba(Vec3::splat(roughness));
        }
        ViewportDisplayMode::MetalnessOnly => {
            return to_u8_rgba(Vec3::splat(metalness));
        }
        ViewportDisplayMode::OcclusionOnly => {
            return to_u8_rgba(Vec3::splat(ao));
        }
        ViewportDisplayMode::EmissiveOnly => {
            return to_u8_rgba(emissive);
        }
        ViewportDisplayMode::CustomAOnly => {
            return to_u8_rgba(Vec3::splat(custom_a));
        }
        ViewportDisplayMode::CustomBOnly => {
            return to_u8_rgba(custom_b);
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

    // D: GGX
    let alpha2 = alpha * alpha;
    let d_denom = n_dot_h * n_dot_h * (alpha2 - 1.0) + 1.0;
    let d = alpha2 / (std::f32::consts::PI * d_denom * d_denom);

    // F: Schlick
    let f = f0 + (Vec3::ONE - f0) * (1.0 - v_dot_h).powi(5);

    // G: Smith
    let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
    let g1_l = n_dot_l / (n_dot_l * (1.0 - k) + k);
    let g1_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
    let g = g1_l * g1_v;

    let specular_brdf = (d * g * f) / (4.0 * n_dot_l * n_dot_v).max(1e-4);
    let kd = (Vec3::ONE - f) * (1.0 - metalness);
    let diffuse_brdf = kd * albedo / std::f32::consts::PI;

    let direct_lighting = (diffuse_brdf + specular_brdf) * light_color * n_dot_l;

    // Ambient
    let up_factor = normal.y * 0.5 + 0.5;
    let ambient_light = ambient_ground.lerp(ambient_sky, up_factor);
    let ambient = (kd * albedo + f0 * (1.0 - roughness)) * ambient_light * ao;

    // Custom Shading Model Modifications matching Ray-MMD ReForge
    let mut custom_lighting = Vec3::ZERO;
    match shading_model {
        ShadingModel::Skin | ShadingModel::Subsurface => {
            // Curvature (Custom A) softens shadows with warm scatter bleed (Custom B)
            let scatter = (1.0 - n_dot_l) * custom_a * 0.35;
            custom_lighting += custom_b * albedo * scatter;
        }
        ShadingModel::Cloth => {
            // Sheen (Custom A) and Fuzz (Custom B)
            let sheen_fresnel = (1.0 - n_dot_v).powi(4) * custom_a;
            custom_lighting += custom_b * sheen_fresnel * light_color * 0.6;
        }
        ShadingModel::ClearCoat => {
            // Secondary glossy clearcoat specular
            let cc_rough = (1.0 - custom_a).clamp(0.04, 1.0);
            let cc_alpha = cc_rough * cc_rough;
            let cc_d = cc_alpha / (std::f32::consts::PI * (n_dot_h * n_dot_h * (cc_alpha - 1.0) + 1.0).powi(2));
            let cc_f = 0.04 + (1.0 - 0.04) * (1.0 - v_dot_h).powi(5);
            let cc_spec = cc_d * cc_f / (4.0 * n_dot_l * n_dot_v).max(1e-4);
            custom_lighting += Vec3::splat(cc_spec * n_dot_l * 0.8);
        }
        ShadingModel::CelShading => {
            // Shadow threshold from Custom A, tinted with Custom B
            let threshold = custom_a.clamp(0.01, 0.99);
            let ramp = if n_dot_l > threshold { 1.0 } else { 0.25 };
            custom_lighting = (custom_b - Vec3::ONE) * (1.0 - ramp) * albedo;
        }
        ShadingModel::Anisotropy => {
            let aniso_factor = (1.0 - normal.x.abs() * custom_a).clamp(0.2, 1.0);
            custom_lighting = albedo * aniso_factor * 0.2;
        }
        _ => {}
    }

    let mut total_color = direct_lighting + ambient + emissive + custom_lighting;

    // Active subset subtle highlight tint
    if is_active_subset {
        total_color = total_color * 1.1 + Vec3::new(0.05, 0.1, 0.15);
    }

    // Reinhard tonemapping + gamma 2.2
    let tonemapped = total_color / (total_color + Vec3::ONE);
    let gamma = tonemapped.powf(1.0 / 2.2);

    to_u8_rgba(gamma)
}

#[inline(always)]
fn to_u8_rgba(color: Vec3) -> [u8; 4] {
    [
        (color.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        (color.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        (color.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        255,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_pmx_rasterizer() {
        let pmx_path = Path::new(r"..\..\Materials\Editor\Anisotropic\material_aniso_1.pmx");
        if !pmx_path.exists() {
            return;
        }

        let model = crate::pmx::PmxModel::load_from_file(pmx_path).expect("Failed to load test PMX");
        assert!(!model.vertices.is_empty(), "Model has vertices");
        assert!(!model.indices.is_empty(), "Model has indices");

        let camera = PreviewCamera {
            distance: model.radius * 2.5,
            pitch: 0.1,
            yaw: 0.0,
            ..Default::default()
        };

        let sub_mats = vec![None; model.subsets.len()];
        let fallback_tex = vec![None; model.subsets.len()];

        let img = render_pmx_model(
            &model,
            &sub_mats,
            &fallback_tex,
            &camera,
            ShadingModel::Default,
            ViewportDisplayMode::FullPbr,
            None,
            false,
            256,
            256,
        );

        assert_eq!(img.width(), 256);
        assert_eq!(img.height(), 256);

        // Check that at least some pixels were rendered
        let mut non_bg_count = 0;
        for pix in img.pixels() {
            // Background is dark blueish-grey (pix[0] around 20-30, pix[1] around 25-35, pix[2] around 30-40)
            // Model pixels will have distinct values
            if pix[0] > 50 || pix[1] > 50 || pix[2] > 50 {
                non_bg_count += 1;
            }
        }
        assert!(non_bg_count > 100, "Expected model pixels rendered, got {}", non_bg_count);
    }

    #[test]
    fn test_near_plane_clipping_floor() {
        use crate::pmx::PmxVertex;

        // Construct a stage floor quad from Z = -20.0 to +20.0, camera at Y = 2.0 looking down Z+
        // Two vertices are behind the camera (Z = -20.0), two are in front (Z = +20.0)
        let vertices = vec![
            PmxVertex { position: Vec3::new(-10.0, 0.0, -20.0), normal: Vec3::Y, uv: Vec2::new(0.0, 0.0) },
            PmxVertex { position: Vec3::new( 10.0, 0.0, -20.0), normal: Vec3::Y, uv: Vec2::new(1.0, 0.0) },
            PmxVertex { position: Vec3::new( 10.0, 0.0,  20.0), normal: Vec3::Y, uv: Vec2::new(1.0, 1.0) },
            PmxVertex { position: Vec3::new(-10.0, 0.0,  20.0), normal: Vec3::Y, uv: Vec2::new(0.0, 1.0) },
        ];
        // 2 triangles forming quad: (0, 1, 2) and (0, 2, 3)
        let indices = vec![0, 1, 2, 0, 2, 3];
        let subset = PmxSubset {
            index: 0,
            name_local: "Floor".to_string(),
            name_universal: "Floor".to_string(),
            diffuse: [1.0, 1.0, 1.0, 1.0],
            specular: [0.0, 0.0, 0.0],
            ambient: [0.1, 0.1, 0.1],
            is_both_faces: true,
            texture_index: None,
            texture_path: None,
            absolute_texture_path: None,
            index_start: 0,
            index_count: 6,
            is_visible: true,
        };
        let model = PmxModel {
            name_local: "FloorModel".to_string(),
            name_universal: "FloorModel".to_string(),
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 20.0,
            vertices,
            indices,
            subsets: vec![subset],
            ..Default::default()
        };

        let camera = PreviewCamera {
            target: Vec3::new(0.0, 0.0, 5.0),
            distance: 4.0,
            pitch: 0.2,
            yaw: 0.0,
            ..Default::default()
        };

        let img = render_pmx_model(
            &model,
            &[None],
            &[None],
            &camera,
            ShadingModel::Default,
            ViewportDisplayMode::FullPbr,
            None,
            false,
            128,
            128,
        );

        let mut rendered_floor_pixels = 0;
        for pix in img.pixels() {
            if pix[0] > 40 || pix[1] > 40 || pix[2] > 40 {
                rendered_floor_pixels += 1;
            }
        }
        assert!(rendered_floor_pixels > 200, "Floor must be rendered with near-plane clipping! Got {} pixels", rendered_floor_pixels);
    }

    #[test]
    fn test_load_cafe_stage() {
        let path = Path::new(r"C:\Users\Norz3n\AppData\Roaming\MikuMikuDanceE_v932x64\UserFile\Model\Stages\CM3D2 - Cafe\Cafe(without_sky).pmx");
        if !path.exists() {
            println!("Cafe path does not exist, skipping");
            return;
        }

        match PmxModel::load_from_file(path) {
            Ok(model) => {
                println!("SUCCESS loaded Cafe: {} vertices, {} indices, {} subsets",
                    model.vertices.len(), model.indices.len(), model.subsets.len());
                let view = Mat4::look_at_lh(Vec3::new(0.0, 10.0, -20.0), Vec3::new(0.0, 10.0, 0.0), Vec3::Y);
                let proj = Mat4::perspective_lh(45.0f32.to_radians(), 1.0, 0.2, 1000.0);
                let p_in_front = Vec4::new(0.0, 10.0, 0.0, 1.0); // distance 20 in front of camera
                let p_behind = Vec4::new(0.0, 10.0, -30.0, 1.0); // 10 behind camera
                let clip_front = proj * (view * p_in_front);
                let clip_behind = proj * (view * p_behind);
                println!("clip_front = {:?}, w={}", clip_front, clip_front.w);
                println!("clip_behind = {:?}, w={}", clip_behind, clip_behind.w);
                println!("proj = {:?}", proj);



                let sub0 = &model.subsets[0];
                println!("Subset 0: name='{}', start={}, count={}, tri_count={}",
                    sub0.name_local, sub0.index_start, sub0.index_count, sub0.index_count / 3);

                let camera = PreviewCamera {
                    target: Vec3::new(0.0, 8.0, 10.0),
                    distance: 30.0,
                    pitch: 0.18,
                    yaw: 0.3,
                    ..Default::default()
                };

                let mut fallback_textures: Vec<Option<U8Image>> = Vec::new();
                for s in &model.subsets {
                    if let Some(ref tp) = s.absolute_texture_path {
                        if let Ok(dyn_img) = image::open(tp) {
                            fallback_textures.push(Some(dyn_img.to_rgba8()));
                        } else {
                            fallback_textures.push(None);
                        }
                    } else {
                        fallback_textures.push(None);
                    }
                }
                let sub_mats = vec![None; model.subsets.len()];

                let img = render_pmx_model(
                    &model,
                    &sub_mats,
                    &fallback_textures,
                    &camera,
                    ShadingModel::Default,
                    ViewportDisplayMode::FullPbr,
                    None,
                    false,
                    640,
                    360,
                );

                let out_path = std::env::temp_dir().join("cafe_close.png");
                let _ = image::save_buffer(
                    &out_path,
                    img.as_raw(),
                    img.width(),
                    img.height(),
                    image::ExtendedColorType::Rgba8,
                );
                println!("Saved cafe_close render to {:?}", out_path);
            }
            Err(e) => {
                panic!("FAILED to load Cafe model: {}", e);
            }
        }
    }
}
