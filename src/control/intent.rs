use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotionIntent {
    pub vx: f32,
    pub vy: f32,
    pub yaw_rate: f32,
    pub confidence: f32,
    pub timestamp_ms: u64,
}

impl MotionIntent {
    pub fn zero(ts: u64) -> Self {
        Self { vx: 0.0, vy: 0.0, yaw_rate: 0.0, confidence: 0.0, timestamp_ms: ts }
    }
}
