#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

pub mod error;
pub mod formats;
pub mod gui;

use eframe::egui;
use std::collections::{HashMap, VecDeque};

use crate::gui::MancieGui;

fn main() -> eframe::Result {
    env_logger::init();
    let mut errors: VecDeque<crate::error::Error> = VecDeque::new();
    let dependencies = get_dependency_binaries(&["python3", "blender"], &mut errors);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "mancie",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MancieGui {
                errors,
                ..Default::default()
            }))
        }),
    )
}
fn get_dependency_binaries(
    dependencies: &[&str],
    errors: &mut VecDeque<crate::error::Error>,
) -> HashMap<String, String> {
    dependencies
        .iter()
        .map(|d| (d, d))
        .map(|(f, d)| (f, which::which(d)))
        .filter_map(|(f, d)| {
            d.inspect_err(|_| {
                errors.push_back(crate::error::Error::missing_executable(String::from(*f)))
            })
            .map(|d| (String::from(*f), String::from(d.to_str().unwrap())))
            .ok()
        })
        .collect()
}
