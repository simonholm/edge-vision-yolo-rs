# edge-vision-yolo-rs

A small Rust prototype for **edge computer vision** using OpenCV and (eventually) ONNX Runtime.

This project focuses on **toolchain stability and architecture first**, especially on constrained systems such as **Android / Termux**.

---

## Status

✅ Builds cleanly on Termux  
✅ OpenCV Rust bindings working  
⏸ ONNX Runtime intentionally stubbed  

See `ARCHITECTURE.md` for details.

---

## Goals

- Explore edge-vision pipelines in Rust
- Validate OpenCV on mobile / edge systems
- Incrementally add ONNX (YOLO) inference
- Keep the codebase understandable and modular

---

## Non-Goals

- Military or weaponized systems
- Autonomous targeting
- Production readiness (yet)

---

## Build

```bash
cargo build

## Bench-first development

This repository now supports a bench-first workflow.

The bench binary allows testing:
- motion intent generation
- safety constraints
- control output formatting

without requiring flight hardware or ONNX Runtime.

### Termux note

Termux is supported for control and bench development.
ONNX Runtime builds are unreliable on Android/Termux and are not required
for bench validation.

Full object detection is expected to run on Jetson or Linux hosts.
