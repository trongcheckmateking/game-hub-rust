//! Small drawing and outcome helpers for turn-based games.
use crate::core::game::{ResultState, Round};
use macroquad::prelude::*;
pub fn bg(hex: u32) {
    draw_rectangle(0., 0., 960., 540., Color::from_hex(hex));
}
pub fn caption(s: &str, x: f32, y: f32, size: f32, c: Color) {
    draw_text(s, x, y, size, c);
}
pub fn turn(player: usize, solo: bool) {
    let s = if solo && player == 1 {
        "CPU".into()
    } else {
        format!("P{}", player + 1)
    };
    caption(&s, 25., 48., 32., super::color(player));
}
pub fn finish(r: &mut Round, p: usize) {
    r.result = if p < r.score.len() {
        ResultState::Winner(p)
    } else {
        ResultState::Defeat
    };
}
pub fn circle(x: f32, y: f32, r: f32, c: Color) {
    draw_circle(x + 2., y + 3., r, Color::new(0., 0., 0., 0.25));
    draw_circle(x, y, r, c);
}
