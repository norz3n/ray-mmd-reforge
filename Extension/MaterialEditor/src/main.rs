//! ReForge Material Editor — High-performance node-based PBR material authoring and ShaderMap generator for Ray-MMD ReForge.

#![windows_subsystem = "windows"]

pub mod app;
pub mod graph;
pub mod image_proc;
pub mod material_export;
pub mod pmx;
pub mod viewport;


use app::MaterialEditorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // High DPI and native window configuration
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ReForge Material Editor — Ray-MMD (PBR Node Graph & ShaderMap)")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([960.0, 600.0])
            .with_active(true),
        ..Default::default()
    };

    eframe::run_native(
        "ReForge Material Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(MaterialEditorApp::new(cc)))),
    )
}
