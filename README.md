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
