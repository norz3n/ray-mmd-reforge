//! ReForge Material Editor — High-performance node-based PBR material authoring and ShaderMap generator for Ray-MMD ReForge.

#![windows_subsystem = "windows"]

pub mod app;
pub mod graph;
pub mod image_proc;
pub mod material_export;
pub mod pmx;
pub mod viewport;


use std::sync::Arc;
use app::MaterialEditorApp;
use eframe::egui;

/// Loads application icon from the embedded ReForge branding logo.
fn load_app_icon() -> Option<Arc<egui::IconData>> {
    let img_bytes = include_bytes!("../../../docs/pics/branding/reforge_logo.webp");
    let img = image::load_from_memory(img_bytes).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(Arc::new(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    }))
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("ReForge Material Editor — Ray-MMD (PBR Node Graph & ShaderMap)")
        .with_inner_size([1440.0, 900.0])
        .with_min_inner_size([960.0, 600.0])
        .with_active(true);

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    // High DPI and native window configuration
    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "ReForge Material Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(MaterialEditorApp::new(cc)))),
    )
}
