//! High-performance PMX (Polygon Model eXtended) 2.0 / 2.1 parser for MMD models.
//!
//! Extracts:
//! - Model information (Japanese & English name and comments)
//! - Vertex buffer (positions, normals, UVs)
//! - Face index buffer
//! - Texture file paths list
//! - Material subsets (local/universal name, diffuse, specular, ambient, texture index, face index ranges)
//! - Bounding box for camera framing

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use glam::{Vec2, Vec3};

pub mod renderer;
pub use renderer::*;


/// Header metadata and index size settings from PMX global flags.
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PmxGlobals {
    text_encoding: u8,      // 0: UTF-16LE, 1: UTF-8
    additional_vec4s: u8,   // 0..4
    vertex_index_size: u8,  // 1, 2, or 4 bytes
    texture_index_size: u8, // 1, 2, or 4 bytes
    material_index_size: u8,// 1, 2, or 4 bytes
    bone_index_size: u8,    // 1, 2, or 4 bytes
    morph_index_size: u8,   // 1, 2, or 4 bytes
    rigidbody_index_size: u8,// 1, 2, or 4 bytes
}

/// A single vertex from the PMX model.
#[derive(Debug, Clone, Copy, Default)]
pub struct PmxVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

/// A material subset in the PMX model.
#[derive(Debug, Clone)]
pub struct PmxSubset {
    pub index: usize,
    pub name_local: String,
    pub name_universal: String,
    pub diffuse: [f32; 4],
    pub specular: [f32; 3],
    pub ambient: [f32; 3],
    pub is_both_faces: bool,
    pub texture_index: Option<usize>,
    pub texture_path: Option<String>,
    pub absolute_texture_path: Option<PathBuf>,
    pub index_start: usize,
    pub index_count: usize,
    pub is_visible: bool,
}

impl PmxSubset {
    pub fn display_name(&self) -> &str {
        if !self.name_universal.is_empty() {
            &self.name_universal
        } else if !self.name_local.is_empty() {
            &self.name_local
        } else {
            "Unnamed_Subset"
        }
    }
}

/// Translates common Japanese PMX material keywords to clean English fallback names
/// so that subsets don't render as missing glyph boxes in egui.
pub fn sanitize_pmx_subset_name(name_local: &str, name_universal: &str, index: usize) -> String {
    let trimmed_univ = name_universal.trim();
    if !trimmed_univ.is_empty() && trimmed_univ.chars().all(|c| c.is_ascii() || !c.is_control()) {
        if trimmed_univ.chars().any(|c| c.is_ascii_alphanumeric()) {
            return trimmed_univ.to_string();
        }
    }

    let trimmed_local = name_local.trim();
    if trimmed_local.is_empty() {
        return format!("Material_{:02}", index);
    }

    // Common MMD / PMX stage and model material translations
    let mut clean = trimmed_local.to_string();
    let translations = [
        ("材質", "Material_"),
        ("マテリアル", "Material_"),
        ("床", "Floor"),
        ("壁", "Wall"),
        ("天井", "Ceiling"),
        ("窓", "Window"),
        ("ガラス", "Glass"),
        ("ドア", "Door"),
        ("机", "Desk"),
        ("テーブル", "Table"),
        ("椅子", "Chair"),
        ("柱", "Pillar"),
        ("屋根", "Roof"),
        ("外観", "Exterior"),
        ("内装", "Interior"),
        ("階段", "Stairs"),
        ("手すり", "Handrail"),
        ("本", "Book"),
        ("本棚", "Bookshelf"),
        ("照明", "Light"),
        ("ランプ", "Lamp"),
        ("カーテン", "Curtain"),
        ("ソファ", "Sofa"),
        ("フェンス", "Fence"),
        ("金属", "Metal"),
        ("木", "Wood"),
        ("布", "Cloth"),
        ("影", "Shadow"),
        ("地面", "Ground"),
        ("肌", "Skin"),
        ("髪", "Hair"),
        ("目", "Eye"),
        ("瞳", "Pupil"),
        ("白目", "Sclera"),
        ("眉", "Eyebrow"),
        ("まつげ", "Eyelashes"),
        ("口", "Mouth"),
        ("歯", "Teeth"),
        ("舌", "Tongue"),
        ("顔", "Face"),
        ("体", "Body"),
        ("服", "Clothes"),
        ("スカート", "Skirt"),
        ("靴", "Shoes"),
        ("リボン", "Ribbon"),
    ];

    for (jp, en) in translations {
        if clean.contains(jp) {
            clean = clean.replace(jp, en);
        }
    }

    // If still containing non-ascii characters (unmapped CJK kanji/kana), provide safe fallback
    if clean.chars().any(|c| c as u32 > 127) {
        let digits: String = clean.chars().filter(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            format!("Material_{}", digits)
        } else {
            format!("Material_{:02}", index)
        }
    } else {
        clean
    }
}

/// Fully parsed PMX model geometry and subset definitions.
#[derive(Debug, Clone, Default)]
pub struct PmxModel {
    pub file_path: Option<PathBuf>,
    pub name_local: String,
    pub name_universal: String,
    pub comment_local: String,
    pub comment_universal: String,
    pub vertices: Vec<PmxVertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<String>,
    pub subsets: Vec<PmxSubset>,
    pub bbox_min: Vec3,
    pub bbox_max: Vec3,
    pub center: Vec3,
    pub radius: f32,
}

impl PmxModel {
    /// Loads and parses a PMX model from disk.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref).map_err(|e| format!("Cannot open PMX file: {}", e))?;
        let mut reader = BufReader::new(file);
        let base_dir = path_ref.parent().unwrap_or_else(|| Path::new("."));

        Self::parse(&mut reader, base_dir, Some(path_ref.to_path_buf()))
    }

    /// Internal parser from any reader.
    fn parse<R: Read + Seek>(
        reader: &mut R,
        base_dir: &Path,
        file_path: Option<PathBuf>,
    ) -> Result<Self, String> {
        // 1. Magic check: b"PMX "
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).map_err(|e| format!("Failed to read magic: {}", e))?;
        if &magic != b"PMX " {
            return Err("Invalid PMX magic header. File is not a valid PMX model.".to_string());
        }

        // 2. Version: f32 (2.0 or 2.1)
        let mut ver_buf = [0u8; 4];
        reader.read_exact(&mut ver_buf).map_err(|e| format!("Failed to read version: {}", e))?;
        let _version = f32::from_le_bytes(ver_buf);

        // 3. Globals count and flags
        let mut globals_count = [0u8; 1];
        reader.read_exact(&mut globals_count).map_err(|e| format!("Failed to read globals count: {}", e))?;
        let g_len = globals_count[0] as usize;
        if g_len < 8 {
            return Err(format!("Invalid PMX globals length: {}", g_len));
        }

        let mut globals_bytes = vec![0u8; g_len];
        reader.read_exact(&mut globals_bytes).map_err(|e| format!("Failed to read globals flags: {}", e))?;

        let globals = PmxGlobals {
            text_encoding: globals_bytes[0],
            additional_vec4s: globals_bytes[1],
            vertex_index_size: globals_bytes[2],
            texture_index_size: globals_bytes[3],
            material_index_size: globals_bytes[4],
            bone_index_size: globals_bytes[5],
            morph_index_size: globals_bytes[6],
            rigidbody_index_size: globals_bytes[7],
        };

        // 4. Model info strings
        let name_local = read_pmx_text(reader, globals.text_encoding)?;
        let name_universal = read_pmx_text(reader, globals.text_encoding)?;
        let comment_local = read_pmx_text(reader, globals.text_encoding)?;
        let comment_universal = read_pmx_text(reader, globals.text_encoding)?;

        // 5. Vertices
        let vert_count = read_i32(reader)? as usize;
        let mut vertices = Vec::with_capacity(vert_count);

        let mut bbox_min = Vec3::splat(f32::MAX);
        let mut bbox_max = Vec3::splat(f32::MIN);

        for _ in 0..vert_count {
            let pos_x = read_f32(reader)?;
            let pos_y = read_f32(reader)?;
            let pos_z = read_f32(reader)?;
            let position = Vec3::new(pos_x, pos_y, pos_z);

            let n_x = read_f32(reader)?;
            let n_y = read_f32(reader)?;
            let n_z = read_f32(reader)?;
            let normal = Vec3::new(n_x, n_y, n_z);

            let u = read_f32(reader)?;
            let v = read_f32(reader)?;
            let uv = Vec2::new(u, v);

            // Skip additional vec4s
            if globals.additional_vec4s > 0 {
                let skip_bytes = globals.additional_vec4s as i64 * 16;
                reader.seek(SeekFrom::Current(skip_bytes)).map_err(|e| format!("Seek failed: {}", e))?;
            }

            // Bone deform type
            let mut deform_type = [0u8; 1];
            reader.read_exact(&mut deform_type).map_err(|e| format!("Read deform failed: {}", e))?;
            match deform_type[0] {
                0 => { // BDEF1
                    let skip = globals.bone_index_size as i64;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
                1 => { // BDEF2
                    let skip = globals.bone_index_size as i64 * 2 + 4;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
                2 => { // BDEF4
                    let skip = globals.bone_index_size as i64 * 4 + 16;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
                3 => { // SDEF
                    let skip = globals.bone_index_size as i64 * 2 + 4 + 12 + 12 + 12;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
                4 => { // QDEF
                    let skip = globals.bone_index_size as i64 * 4 + 16;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
                _ => {
                    let skip = globals.bone_index_size as i64;
                    reader.seek(SeekFrom::Current(skip)).map_err(|e| format!("Seek failed: {}", e))?;
                }
            }

            // Edge scale: f32
            let _edge_scale = read_f32(reader)?;

            bbox_min = bbox_min.min(position);
            bbox_max = bbox_max.max(position);

            vertices.push(PmxVertex {
                position,
                normal,
                uv,
            });
        }

        if vertices.is_empty() {
            bbox_min = Vec3::ZERO;
            bbox_max = Vec3::ZERO;
        }

        let center = (bbox_min + bbox_max) * 0.5;
        let radius = (bbox_max - bbox_min).length() * 0.5;

        // 6. Faces / Surface Indices
        let face_index_count = read_i32(reader)? as usize;
        let mut indices = Vec::with_capacity(face_index_count);

        for _ in 0..face_index_count {
            let idx = read_sized_uint(reader, globals.vertex_index_size)?;
            indices.push(idx);
        }

        // 7. Textures table
        let tex_count = read_i32(reader)? as usize;
        let mut textures = Vec::with_capacity(tex_count);
        for _ in 0..tex_count {
            let tex_str = read_pmx_text(reader, globals.text_encoding)?;
            // Normalize path separators to forward slash
            let normalized = tex_str.replace('\\', "/");
            textures.push(normalized);
        }

        // 8. Materials / Subsets
        let mat_count = read_i32(reader)? as usize;
        let mut subsets = Vec::with_capacity(mat_count);
        let mut current_index_offset = 0usize;

        for m_idx in 0..mat_count {
            let m_name_local = read_pmx_text(reader, globals.text_encoding)?;
            let m_name_universal = read_pmx_text(reader, globals.text_encoding)?;

            let diff_r = read_f32(reader)?;
            let diff_g = read_f32(reader)?;
            let diff_b = read_f32(reader)?;
            let diff_a = read_f32(reader)?;
            let diffuse = [diff_r, diff_g, diff_b, diff_a];

            let spec_r = read_f32(reader)?;
            let spec_g = read_f32(reader)?;
            let spec_b = read_f32(reader)?;
            let specular = [spec_r, spec_g, spec_b];

            let _spec_power = read_f32(reader)?;

            let amb_r = read_f32(reader)?;
            let amb_g = read_f32(reader)?;
            let amb_b = read_f32(reader)?;
            let ambient = [amb_r, amb_g, amb_b];

            let draw_flags = read_u8(reader)?;
            let is_both_faces = (draw_flags & 0x01) != 0;

            // Edge color & size
            let _edge_r = read_f32(reader)?;
            let _edge_g = read_f32(reader)?;
            let _edge_b = read_f32(reader)?;
            let _edge_a = read_f32(reader)?;
            let _edge_size = read_f32(reader)?;

            // Texture index
            let tex_idx_raw = read_sized_int(reader, globals.texture_index_size)?;
            let texture_index = if tex_idx_raw >= 0 && (tex_idx_raw as usize) < textures.len() {
                Some(tex_idx_raw as usize)
            } else {
                None
            };

            // Sphere texture index & mode
            let _sphere_tex_idx = read_sized_int(reader, globals.texture_index_size)?;
            let _sphere_mode = read_u8(reader)?;

            // Shared toon flag & toon index
            let shared_toon = read_u8(reader)?;
            if shared_toon == 0 {
                let _toon_idx = read_sized_int(reader, globals.texture_index_size)?;
            } else {
                let _toon_byte = read_u8(reader)?;
            }

            // Memo string
            let _memo = read_pmx_text(reader, globals.text_encoding)?;

            // Surface index count for this material
            let subset_index_count = read_i32(reader)? as usize;

            let texture_path = texture_index.and_then(|i| textures.get(i).cloned());
            let absolute_texture_path = texture_path.as_ref().map(|p| {
                let mut full = base_dir.to_path_buf();
                for comp in p.split('/') {
                    full.push(comp);
                }
                full
            });

            let clean_universal = if m_name_universal.trim().is_empty() || m_name_universal.chars().any(|c| c as u32 > 127) {
                sanitize_pmx_subset_name(&m_name_local, &m_name_universal, m_idx)
            } else {
                m_name_universal
            };

            subsets.push(PmxSubset {
                index: m_idx,
                name_local: m_name_local,
                name_universal: clean_universal,
                diffuse,
                specular,
                ambient,
                is_both_faces,
                texture_index,
                texture_path,
                absolute_texture_path,
                index_start: current_index_offset,
                index_count: subset_index_count,
                is_visible: true,
            });

            current_index_offset += subset_index_count;
        }

        Ok(Self {
            file_path,
            name_local,
            name_universal,
            comment_local,
            comment_universal,
            vertices,
            indices,
            textures,
            subsets,
            bbox_min,
            bbox_max,
            center,
            radius: radius.max(0.1),
        })
    }
}

// ----------------------------------------------------------------------------
// Low-level binary helpers
// ----------------------------------------------------------------------------

#[inline(always)]
fn read_u8<R: Read>(reader: &mut R) -> Result<u8, String> {
    let mut b = [0u8; 1];
    reader.read_exact(&mut b).map_err(|e| format!("Read u8 error: {}", e))?;
    Ok(b[0])
}

#[inline(always)]
fn read_i32<R: Read>(reader: &mut R) -> Result<i32, String> {
    let mut b = [0u8; 4];
    reader.read_exact(&mut b).map_err(|e| format!("Read i32 error: {}", e))?;
    Ok(i32::from_le_bytes(b))
}

#[inline(always)]
fn read_f32<R: Read>(reader: &mut R) -> Result<f32, String> {
    let mut b = [0u8; 4];
    reader.read_exact(&mut b).map_err(|e| format!("Read f32 error: {}", e))?;
    Ok(f32::from_le_bytes(b))
}

#[inline(always)]
fn read_sized_uint<R: Read>(reader: &mut R, size: u8) -> Result<u32, String> {
    match size {
        1 => {
            let mut b = [0u8; 1];
            reader.read_exact(&mut b).map_err(|e| format!("Read u8 error: {}", e))?;
            Ok(b[0] as u32)
        }
        2 => {
            let mut b = [0u8; 2];
            reader.read_exact(&mut b).map_err(|e| format!("Read u16 error: {}", e))?;
            Ok(u16::from_le_bytes(b) as u32)
        }
        4 => {
            let mut b = [0u8; 4];
            reader.read_exact(&mut b).map_err(|e| format!("Read u32 error: {}", e))?;
            Ok(u32::from_le_bytes(b))
        }
        s => Err(format!("Unsupported uint size: {}", s)),
    }
}

#[inline(always)]
fn read_sized_int<R: Read>(reader: &mut R, size: u8) -> Result<i32, String> {
    match size {
        1 => {
            let mut b = [0u8; 1];
            reader.read_exact(&mut b).map_err(|e| format!("Read i8 error: {}", e))?;
            Ok(b[0] as i8 as i32)
        }
        2 => {
            let mut b = [0u8; 2];
            reader.read_exact(&mut b).map_err(|e| format!("Read i16 error: {}", e))?;
            Ok(i16::from_le_bytes(b) as i32)
        }
        4 => {
            let mut b = [0u8; 4];
            reader.read_exact(&mut b).map_err(|e| format!("Read i32 error: {}", e))?;
            Ok(i32::from_le_bytes(b))
        }
        s => Err(format!("Unsupported int size: {}", s)),
    }
}

fn read_pmx_text<R: Read>(reader: &mut R, encoding: u8) -> Result<String, String> {
    let len = read_i32(reader)? as usize;
    if len == 0 {
        return Ok(String::new());
    }

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).map_err(|e| format!("Read text bytes error: {}", e))?;

    if encoding == 0 {
        // UTF-16LE
        let u16_count = len / 2;
        let mut u16_buf = Vec::with_capacity(u16_count);
        for i in 0..u16_count {
            let lo = buf[i * 2];
            let hi = buf[i * 2 + 1];
            u16_buf.push(u16::from_le_bytes([lo, hi]));
        }
        Ok(String::from_utf16_lossy(&u16_buf))
    } else {
        // UTF-8
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}
