# Architecture – edge-vision-yolo-rs

## Status
- Builds cleanly on Termux (Android)
- OpenCV Rust bindings linked and working
- ONNX Runtime intentionally stubbed

This checkpoint locks in a working toolchain before adding complex native dependencies.

## Modules
- main.rs        : orchestration only
- camera/        : frame acquisition (stubbed)
- detector/      : object detection API (ONNX stub)
- visualizer/    : drawing / overlay (stubbed)
- tracker/       : future temporal tracking
- multiview/     : future multi-camera logic

## Notes
- Civilian prototype
- No autonomous targeting
- ONNX Runtime will be reintroduced later, in isolation
