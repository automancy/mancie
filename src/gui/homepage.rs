use crate::MancieGui;
use eframe::egui::{self, Ui};

pub fn homepage_ui(
    app: &mut MancieGui,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    ui.heading("mancie");
}
