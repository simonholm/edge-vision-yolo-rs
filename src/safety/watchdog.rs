use crate::control::intent::MotionIntent;

pub struct Watchdog {
    pub timeout_ms: u64,
}

impl Watchdog {
    pub fn new(timeout_ms: u64) -> Self {
        Self { timeout_ms }
    }

    pub fn apply(&self, now: u64, intent: MotionIntent, has_target: bool) -> MotionIntent {
        if !has_target {
            return MotionIntent::zero(now);
        }
        if now.saturating_sub(intent.timestamp_ms) > self.timeout_ms {
            return MotionIntent::zero(now);
        }
        intent
    }
}
