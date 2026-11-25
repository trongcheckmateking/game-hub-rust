use super::input::PlayerFrame;
pub struct Config { pub seed: u64 }
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ResultState { Playing, Winner(usize), Draw }
pub trait MiniGame {
    fn init(&mut self, num_players: usize, config: Config);
    fn handle_input(&mut self, players: &[PlayerFrame]);
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn get_score(&self) -> &[u32];
    fn result(&self) -> ResultState;
    fn is_game_over(&self) -> bool { self.result() != ResultState::Playing }
}
