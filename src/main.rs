mod camera;
mod multiview;
mod detector;
mod visualizer;

use anyhow::Result;

fn main() -> Result<()> {
    let mut cam = camera::Camera::new(0)?;
    let detector = detector::Detector::new("models/yolov8n.onnx")?;

    loop {
        let frame = cam.read()?;
        let views = multiview::split_views(&frame, 3);

        for v in views {
            let detections = detector.detect(&v)?;
            visualizer::draw(&v, &detections)?;
        }
    }
}
