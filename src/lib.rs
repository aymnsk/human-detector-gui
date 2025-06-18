use pyo3::prelude::*;
use opencv::{prelude::*, videoio, objdetect, imgproc, core};
use anyhow::Result;
use std::path::Path;

#[pyfunction]
fn detect_humans(input_path: &str, output_path: &str) -> PyResult<()> {
    process_video(input_path, output_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

fn process_video(input_path: &str, output_path: &str) -> Result<()> {
    let mut hog = objdetect::HOGDescriptor::default()?;
    hog.set_svm_detector(&objdetect::get_people_detector()?)?;

    let mut cap = videoio::VideoCapture::from_file(input_path, videoio::CAP_ANY)?;
    let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT)?;
    
    let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    
    let mut out = videoio::VideoWriter::new(
        output_path,
        fourcc,
        fps,
        core::Size::new(width, height),
        true,
    )?;

    let mut frame = Mat::default();
    while cap.read(&mut frame)? {
        let mut frame_gray = Mat::default();
        imgproc::cvt_color(&frame, &mut frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        let mut bboxes = opencv::types::VectorOfRect::new();
        hog.detect(&frame_gray, &mut bboxes, &mut opencv::core::Vector::new())?;

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
    }
    
    Ok(())
}

#[pymodule]
fn human_detector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_humans, m)?;
    Ok(())
}
