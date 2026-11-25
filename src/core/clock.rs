pub const STEP: f32 = 1.0 / 60.0;
#[derive(Default)]
pub struct Clock(f32);
impl Clock {
    pub fn ticks(&mut self, elapsed: f32) -> u8 {
        self.0 += elapsed.clamp(0.0, 0.1);
        let count = (self.0 / STEP).floor() as u8;
        self.0 -= f32::from(count) * STEP;
        count
    }
}
