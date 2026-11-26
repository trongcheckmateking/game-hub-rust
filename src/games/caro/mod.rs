use super::table::*;
use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;
const SIDE: usize = 11;
pub struct Caro {
    r: Round,
    board: [u8; 121],
    cursor: usize,
    actor: usize,
    input: PlayerFrame,
    delay: f32,
}
impl Default for Caro {
    fn default() -> Self {
        Self {
            r: Round::default(),
            board: [0; 121],
            cursor: 60,
            actor: 0,
            input: PlayerFrame::default(),
            delay: 0.,
        }
    }
}
impl Caro {
    fn line(&self, cell: usize, stone: u8) -> u8 {
        let (x, y) = ((cell % SIDE) as i32, (cell / SIDE) as i32);
        [(1, 0), (0, 1), (1, 1), (1, -1)]
            .into_iter()
            .map(|(dx, dy)| {
                let mut n = 1;
                for sign in [-1, 1] {
                    let (mut a, mut b) = (x + sign * dx, y + sign * dy);
                    while (0..11).contains(&a)
                        && (0..11).contains(&b)
                        && self.board[(b * 11 + a) as usize] == stone
                    {
                        n += 1;
                        a += sign * dx;
                        b += sign * dy;
                    }
                }
                n
            })
            .max()
            .unwrap_or(1)
    }
    fn place(&mut self, c: usize) {
        if self.board[c] != 0 {
            return;
        }
        let stone = self.actor as u8 + 1;
        self.board[c] = stone;
        self.r.cues.push(Fx::Tick);
        if self.line(c, stone) >= 5 {
            if self.actor < self.r.score.len() {
                self.r.score[self.actor] = 5;
            }
            finish(&mut self.r, self.actor);
        } else if !self.board.contains(&0) {
            self.r.result = ResultState::Draw;
        } else {
            self.actor = 1 - self.actor;
            self.delay = 0.35;
        }
    }
    fn bot(&self) -> Option<usize> {
        (0..121).filter(|&c| self.board[c] == 0).max_by_key(|&c| {
            let a = self.line(c, 2) as i32;
            let b = self.line(c, 1) as i32;
            let proximity = ((c % SIDE) as i32 - 5).abs() + ((c / SIDE) as i32 - 5).abs();
            (a >= 5, b >= 5, a * a * 10 + b * b * 8 - proximity)
        })
    }
}
impl MiniGame for Caro {
    fn init(&mut self, n: usize, _: Config) {
        *self = Self {
            r: Round::new(n),
            ..Self::default()
        };
    }
    fn round(&self) -> &Round {
        &self.r
    }
    fn round_mut(&mut self) -> &mut Round {
        &mut self.r
    }
    fn handle_input(&mut self, f: &[PlayerFrame]) {
        self.input = f.get(self.actor).copied().unwrap_or_default();
    }
    fn update(&mut self, dt: f32) {
        if !self.r.running() {
            return;
        }
        if self.actor >= self.r.score.len() {
            self.delay -= dt;
            if self.delay <= 0. {
                if let Some(c) = self.bot() {
                    self.place(c);
                }
            }
            return;
        }
        for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            if self.input.pressed[i] {
                let x = (self.cursor % SIDE) as i32;
                let y = (self.cursor / SIDE) as i32;
                self.cursor = ((x + dx).clamp(0, 10) + (y + dy).clamp(0, 10) * 11) as usize;
            }
        }
        if self.input.pressed[4] {
            self.place(self.cursor);
        }
    }
    fn render(&self) {
        bg(0x253b36);
        turn(self.actor, self.r.score.len() == 1);
        draw_rectangle(249., 38., 462., 462., Color::from_hex(0xcaae82));
        for i in 0..11 {
            let t = i as f32 * 42.;
            draw_line(270. + t, 59., 270. + t, 479., 2., BROWN);
            draw_line(270., 59. + t, 690., 59. + t, 2., BROWN);
        }
        for (c, &s) in self.board.iter().enumerate() {
            let (x, y) = (
                270. + (c % SIDE) as f32 * 42.,
                59. + (c / SIDE) as f32 * 42.,
            );
            if s != 0 {
                circle(x, y, 17., if s == 1 { BLACK } else { WHITE });
            }
            if c == self.cursor {
                draw_rectangle_lines(x - 20., y - 20., 40., 40., 4., Color::from_hex(0xb73039));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cpu_blocks_four_on_diagonal() {
        let mut g = Caro::default();
        for c in [0, 12, 24, 36] {
            g.board[c] = 1;
        }
        assert_eq!(g.bot(), Some(48));
    }
}
