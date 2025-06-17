use eframe::egui;
use std::path::PathBuf;

pub struct DetectorApp {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    processing: bool,
    progress: f32,
    message: String,
    vista_theme: VistaTheme,
}

struct VistaTheme {
    button_gradient: egui::Color32,
    window_bg: egui::Color32,
    // ... other theme elements
}

impl Default for DetectorApp {
    fn default() -> Self {
        Self {
            input_path: None,
            output_path: None,
            processing: false,
            progress: 0.0,
            message: String::new(),
            vista_theme: VistaTheme::new(),
        }
    }
}

impl DetectorApp {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.vista_theme.apply(ui);
        
        egui::Window::new("Human Detector (Vista Style)")
            .resizable(true)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Video Human Detector");
                    
                    ui.horizontal(|ui| {
                        if ui.button("Select Input Video").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.input_path = Some(path);
                            }
                        }
                        if let Some(path) = &self.input_path {
                            ui.label(path.display().to_string());
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Select Output Location").clicked() {
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                self.output_path = Some(path);
                            }
                        }
                        if let Some(path) = &self.output_path {
                            ui.label(path.display().to_string());
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    if ui.button("Process Video").clicked() && !self.processing {
                        self.processing = true;
                        // Start processing in another thread
                    }
                    
                    if self.processing {
                        ui.add(egui::ProgressBar::new(self.progress).show_percentage());
                    }
                    
                    if !self.message.is_empty() {
                        ui.label(&self.message);
                    }
                    
                    if let (Some(input), Some(output)) = (&self.input_path, &self.output_path) {
                        if self.processing && self.progress >= 1.0 {
                            if ui.button("Download Processed Video").clicked() {
                                // Implement download logic
                            }
                        }
                    }
                });
            });
    }
}
