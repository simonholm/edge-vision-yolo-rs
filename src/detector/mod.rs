use anyhow::Result;
use opencv::core::Mat;

pub struct Detector;

impl Detector {
    pub fn new(_model_path: &str) -> Result<Self> {
        // ONNX Runtime integration comes later
        Ok(Self)
    }

    pub fn detect(&self, _frame: &Mat) -> Result<()> {
        // YOLO inference wiring comes later
        Ok(())
    }
}
