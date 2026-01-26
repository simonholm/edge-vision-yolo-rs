# Architecture – edge-vision-yolo-rs

## Purpose

This repository is a **civilian edge-vision prototype** written in Rust, intended to explore:

- OpenCV integration on constrained systems (Android / Termux)
- Incremental integration of ONNX Runtime (YOLO-style models)
- A clean, modular structure suitable for edge devices

This is **not** a military system and does not include autonomous targeting logic.

---

## Current State (Checkpoint)

As of this commit:

- The project **builds cleanly on Termux (Android)**.
- OpenCV Rust bindings are **successfully linked and compiled**.
- ONNX Runtime (`ort`) integration is **intentionally stubbed**.

This checkpoint exists to lock in a *working toolchain* before reintroducing complex native dependencies.

---

## Module Overview
