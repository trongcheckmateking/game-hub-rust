/// Fixed simulation delta in seconds.
pub const STEP: f32 = 1.0 / 60.0;

/// Unspent time measured in ticks. Accumulate in f64 so subtracting a
/// six-tick stall cannot make the next full frame look shorter than one tick.
#[derive(Default)]
pub struct Clock(f64);
impl Clock {
    pub fn alpha(&self) -> f32 { self.0 as f32 }
    pub fn ticks(&mut self, elapsed_seconds: f32) -> u8 {
        self.0 += f64::from(elapsed_seconds.clamp(0.0, 0.1)) * 60.0;
        let whole_ticks = self.0.floor() as u8;
        self.0 -= f64::from(whole_ticks);
        whole_ticks
    }
}
