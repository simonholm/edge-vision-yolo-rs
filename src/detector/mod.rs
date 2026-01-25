use anyhow::Result;
use opencv::prelude::*;

pub struct Detector {
    _model_path: String,
}

impl Detector {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            _model_path: path.to_string(),
        })
    }

    pub fn detect(&self, _frame: &Mat) -> Result<Vec<(i32,i32,i32,i32)>> {
        // TODO: integrate ONNX runtime / TensorRT
        Ok(vec![])
    }
}
