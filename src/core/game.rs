use super::input::PlayerFrame;
pub struct Config {
    pub seed: u64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ResultState {
    #[default]
    Playing,
    Winner(usize),
    Defeat,
    Draw,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fx {
    Hit,
    Pickup,
    Explode,
    Lose,
    Tick,
}
#[derive(Default)]
pub struct Round {
    pub score: Vec<u32>,
    pub result: ResultState,
    pub paused: bool,
    pub cues: Vec<Fx>,
}
impl Round {
    pub fn new(n: usize) -> Self {
        Self {
            score: vec![0; n],
            ..Default::default()
        }
    }
    pub fn running(&self) -> bool {
        !self.paused && self.result == ResultState::Playing
    }
    pub fn timed_finish(&mut self, live: &[usize]) {
        let max = live.iter().map(|i| self.score[*i]).max();
        let mut top = live.iter().copied().filter(|i| Some(self.score[*i]) == max);
        self.result = match (top.next(), top.next()) {
            (Some(i), None) => ResultState::Winner(i),
            _ => ResultState::Draw,
        };
    }
}
pub trait MiniGame {
    fn init(&mut self, num_players: usize, config: Config);
    fn handle_input(&mut self, players: &[PlayerFrame]);
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn round(&self) -> &Round;
    fn round_mut(&mut self) -> &mut Round;
    fn pause(&mut self) {
        self.round_mut().paused = true;
    }
    fn resume(&mut self) {
        self.round_mut().paused = false;
    }
    fn get_score(&self) -> &[u32] {
        &self.round().score
    }
    fn is_game_over(&self) -> bool {
        self.round().result != ResultState::Playing
    }
}
pub const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
pub fn overlaps(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
    a.0 < b.0 + b.2 && a.0 + a.2 > b.0 && a.1 < b.1 + b.3 && a.1 + a.3 > b.1
}
pub fn next_random(seed: &mut u64) -> usize {
    *seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*seed >> 32) as usize
}
