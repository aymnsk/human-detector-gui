use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    videoio,
    types::VectorOfMat,
};

pub struct VideoProcessor {
    hog: objdetect::HOGDescriptor,
}

impl VideoProcessor {
    pub fn new() -> anyhow::Result<Self> {
        let mut hog = objdetect::HOGDescriptor::default()?;
        hog.set_svm_detector(&objdetect::get_people_detector()?)?;
        Ok(Self { hog })
    }

    pub fn process_video(&mut self, input_path: &str, output_path: &str) -> anyhow::Result<()> {
        let mut cap = videoio::VideoCapture::from_file(input_path, videoio::CAP_ANY)?;
        let mut frame = Mat::default();
        
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

        while cap.read(&mut frame)? {
            let mut frame_gray = Mat::default();
            imgproc::cvt_color(&frame, &mut frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;
            
            let mut bboxes = VectorOfRect::new();
            let mut weights = VectorOfMat::new();
            self.hog.detect_multi_scale(
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
        }
        
        Ok(())
    }
}
