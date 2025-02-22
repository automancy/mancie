use crate::error::Error;

use eframe::egui;
use std::collections::VecDeque;

pub struct MancieGui {
    pub errors: VecDeque<Error>,
}

impl eframe::App for MancieGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mancie");
            error_modal(ctx, &mut self.errors);
        });
    }
}
fn error_modal(ctx: &egui::Context, errors: &mut VecDeque<Error>) {
    if !errors.is_empty() {
        egui::Window::new("Error")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("Error: {}", errors.front().unwrap().description()));
                if ui.button("Clear").clicked() {
                    errors.pop_front();
                }
            });
    };
}
