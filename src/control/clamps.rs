use super::intent::MotionIntent;

pub struct ClampConfig {
    pub max_vx: f32,
    pub max_vy: f32,
    pub max_yaw_rate: f32,
    pub min_confidence: f32,
}

impl Default for ClampConfig {
    fn default() -> Self {
        Self { max_vx: 0.5, max_vy: 0.5, max_yaw_rate: 0.6, min_confidence: 0.35 }
    }
}

pub fn clamp(intent: MotionIntent, cfg: &ClampConfig) -> MotionIntent {
    if intent.confidence < cfg.min_confidence {
        return MotionIntent::zero(intent.timestamp_ms);
    }
    MotionIntent {
        vx: intent.vx.clamp(-cfg.max_vx, cfg.max_vx),
        vy: intent.vy.clamp(-cfg.max_vy, cfg.max_vy),
        yaw_rate: intent.yaw_rate.clamp(-cfg.max_yaw_rate, cfg.max_yaw_rate),
        ..intent
    }
}
