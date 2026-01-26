mod camera;
mod detector;
mod visualizer;

use anyhow::Result;
use camera::Camera;
use detector::Detector;

fn main() -> Result<()> {
    let camera = Camera::new();
    let detector = Detector::new("models/yolov8n.onnx")?;

    let frame = camera.capture()?;
    detector.detect(&frame)?;

    Ok(())
}
