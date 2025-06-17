use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use eframe::egui;
use opencv::{
    core, highgui, imgproc, objdetect,
    prelude::*,
    videoio,
    types::VectorOfRect,
};
use std::path::PathBuf;
use std::thread;

enum DetectorMessage {
    Progress(f32),
    Done(Result<()>),
}

struct HumanDetectorApp {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    processing: bool,
    progress: f32,
    message: String,
    sender: Option<Sender<DetectorMessage>>,
    receiver: Receiver<DetectorMessage>,
}

impl Default for HumanDetectorApp {
    fn default() -> Self {
        let (sender, receiver) = bounded(10);
        Self {
            input_path: None,
            output_path: None,
            processing: false,
            progress: 0.0,
            message: String::new(),
            sender: Some(sender),
            receiver,
        }
    }
}

impl HumanDetectorApp {
    fn process_video(&mut self) {
        if self.processing || self.input_path.is_none() || self.output_path.is_none() {
            return;
        }

        self.processing = true;
        self.message.clear();
        
        let input = self.input_path.clone().unwrap();
        let output = self.output_path.clone().unwrap();
        let sender = self.sender.take().unwrap();

        thread::spawn(move || {
            if let Err(e) = detect_humans(&input, &output, sender.clone()) {
                let _ = sender.send(DetectorMessage::Done(Err(e)));
            } else {
                let _ = sender.send(DetectorMessage::Done(Ok(())));
            }
        });
    }
}

fn detect_humans(
    input_path: &PathBuf,
    output_path: &PathBuf,
    sender: Sender<DetectorMessage>,
) -> Result<()> {
    let mut hog = objdetect::HOGDescriptor::default()?;
    hog.set_svm_detector(&objdetect::get_people_detector()?)?;

    let input_str = input_path.to_str().unwrap();
    let output_str = output_path.to_str().unwrap();

    let mut cap = videoio::VideoCapture::from_file(input_str, videoio::CAP_ANY)?;
    let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT)?;
    
    let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    
    let mut out = videoio::VideoWriter::new(
        output_str,
        fourcc,
        fps,
        core::Size::new(width, height),
        true,
    )?;

    let mut frame = Mat::default();
    let mut processed_frames = 0;
    
    while cap.read(&mut frame)? {
        let mut frame_gray = Mat::default();
        imgproc::cvt_color(&frame, &mut frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        let mut bboxes = VectorOfRect::new();
        let mut weights = VectorOfMat::new();
        hog.detect_multi_scale(
            &frame_gray,
            &mut bboxes,
            &mut weights,
            0.0,
            core::Size::new(8, 8),
            core::Size::new(0, 0),
            1.05,
            2.0,
            false,
        )?;

        for rect in bboxes {
            imgproc::rectangle(
                &mut frame,
                rect,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }
        
        out.write(&frame)?;
        processed_frames += 1;
        
        let progress = processed_frames as f32 / frame_count as f32;
        let _ = sender.send(DetectorMessage::Progress(progress));
    }
    
    Ok(())
}

impl eframe::App for HumanDetectorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for messages from processing thread
        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                DetectorMessage::Progress(p) => self.progress = p,
                DetectorMessage::Done(result) => {
                    self.processing = false;
                    self.sender = Some(self.receiver.sender().clone());
                    match result {
                        Ok(_) => self.message = "Processing complete!".to_string(),
                        Err(e) => self.message = format!("Error: {}", e),
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Human Detector");
            
            ui.horizontal(|ui| {
                if ui.button("Select Input Video").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.input_path = Some(path);
                    }
                }
                ui.label(self.input_path.as_ref().map_or("No file selected", |p| p.to_str().unwrap()));
            });
            
            ui.horizontal(|ui| {
                if ui.button("Select Output Location").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.output_path = Some(path);
                    }
                }
                ui.label(self.output_path.as_ref().map_or("No file selected", |p| p.to_str().unwrap()));
            });
            
            ui.separator();
            
            if ui.button("Process Video").clicked() && !self.processing {
                self.process_video();
            }
            
            if self.processing {
                ui.label("Processing...");
                ui.add(egui::ProgressBar::new(self.progress).show_percentage());
            }
            
            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });
    }
}

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Human Detector",
        options,
        Box::new(|_cc| Box::new(HumanDetectorApp::default())),
    );
    
    Ok(())
}
