use crate::control::intent::MotionIntent;

pub struct OffboardStub;

impl OffboardStub {
    pub fn send(&self, i: &MotionIntent) {
        println!(\"MAVLINK_STUB vx={:.2} vy={:.2} yaw={:.2} conf={:.2}\",
            i.vx, i.vy, i.yaw_rate, i.confidence);
    }
}
