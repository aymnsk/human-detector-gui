
mod processor;
mod ui;

use crate::ui::DetectorApp;
use eframe::{egui, NativeOptions};

fn main() {
    let options = NativeOptions {
        decorated: true,
        transparent: true,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Human Detector",
        options,
        Box::new(|_cc| Box::new(DetectorApp::default())),
    );
}
