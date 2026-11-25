#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerFrame {
    pub held: [bool; 6],
    pub pressed: [bool; 6],
}
