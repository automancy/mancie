use crate::error::Error;

use eframe::egui::{self, popup, AboveOrBelow, Id, PopupCloseBehavior, Ui};
use std::collections::VecDeque;

#[derive(Default)]
pub struct MancieGui {
    pub errors: VecDeque<Error>,
    pub selected_tab: SelectedTab,
}

impl eframe::App for MancieGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mancie");
            error_modal(ctx, &mut self.errors);
            navbar(ctx, self, ui);
            match self.selected_tab {
                SelectedTab::Main => {
                    ui.heading("mancie");
                }
                SelectedTab::FormatsBlendToGltf => {
                    ui.heading("Blend to GLTF");
                }
                SelectedTab::FormatsSvgToBlend => {
                    ui.heading("SVG to Blend");
                }
            }
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
fn navbar(ctx: &egui::Context, app: &mut MancieGui, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if ui.button("Home").clicked() {
            app.selected_tab = SelectedTab::Main;
        }
        let response = ui.button("Format Converters");
        popup::popup_above_or_below_widget(
            ui,
            Id::new("formats_dropdown"),
            &response,
            AboveOrBelow::Below,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                if ui.button("Blend to GLTF").clicked() {
                    app.selected_tab = SelectedTab::FormatsBlendToGltf;
                }
                if ui.button("SVG to Blend").clicked() {
                    app.selected_tab = SelectedTab::FormatsSvgToBlend;
                }
            },
        );
        if response.clicked() {
            ctx.memory_mut(|m| m.toggle_popup(Id::new("formats_dropdown")));
        }
    });
}
#[derive(Default)]
pub enum SelectedTab {
    #[default]
    Main,
    FormatsBlendToGltf,
    FormatsSvgToBlend,
}
