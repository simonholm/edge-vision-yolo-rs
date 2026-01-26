use opencv::prelude::*;

pub struct Camera;

impl Camera {
    pub fn new() -> Self {
        Self
    }

    pub fn capture(&self) -> opencv::Result<Mat> {
        // Stub: return empty frame for now
        Ok(Mat::default())
    }
}
