# Bench-first roadmap

This project is moving to a bench-first validation model.

Goals:
- Validate perception → intent → safety → control without flight
- Decouple autonomy logic from model runtime constraints
- Support development on constrained environments (Termux)

Bench mode intentionally does NOT require:
- ONNX Runtime
- GPU acceleration
- Camera drivers

YOLO / ONNX Runtime is treated as a deploy-time dependency
and is expected to run on Jetson or standard Linux.

Termux status:
- Control logic: supported
- Bench logic: supported
- ONNX Runtime: unreliable / unsupported

This is a known platform limitation, not a project defect.
