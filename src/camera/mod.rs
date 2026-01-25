use opencv::{videoio, prelude::*};
use anyhow::Result;

pub struct Camera {
    cap: videoio::VideoCapture,
}

impl Camera {
    pub fn new(index: i32) -> Result<Self> {
        let mut cap = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;
        Ok(Self { cap })
    }

    pub fn read(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;
        Ok(frame)
    }
}
