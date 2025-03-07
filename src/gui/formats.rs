use crate::MancieGui;
use eframe::egui::{self, Ui};
pub fn blend_to_gltf_ui(
    app: &mut MancieGui,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    ui.heading("Blend to GLTF");
}
pub fn svg_to_blend_ui(
    app: &mut MancieGui,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    ui.heading("SVG to Blend");
}
