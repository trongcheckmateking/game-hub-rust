use super::table::*;
use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;
const SOLVED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
#[derive(Default)]
pub struct Tiles {
    r: Round,
    cells: [u8; 16],
    blank: usize,
    moves: u32,
    input: PlayerFrame,
}
impl Tiles {
    fn slide(&mut self, d: usize) -> bool {
        let (dx, dy) = DIRECTIONS[d];
        let x = (self.blank % 4) as i32 + dx;
        let y = (self.blank / 4) as i32 + dy;
        if !(0..4).contains(&x) || !(0..4).contains(&y) {
            return false;
        }
        let next = (y * 4 + x) as usize;
        self.cells.swap(self.blank, next);
        self.blank = next;
        true
    }
}
impl MiniGame for Tiles {
    fn init(&mut self, n: usize, c: Config) {
        *self = Self {
            r: Round::new(n),
            cells: SOLVED,
            blank: 15,
            ..Self::default()
        };
        let mut seed = c.seed;
        let mut last = 5;
        for _ in 0..55 {
            loop {
                let d = next_random(&mut seed) % 4;
                if d ^ 1 != last && self.slide(d) {
                    last = d;
                    break;
                }
            }
        }
        if self.cells == SOLVED {
            self.slide(0);
        }
    }
    fn round(&self) -> &Round {
        &self.r
    }
    fn round_mut(&mut self) -> &mut Round {
        &mut self.r
    }
    fn handle_input(&mut self, f: &[PlayerFrame]) {
        self.input = f[0];
    }
    fn update(&mut self, _: f32) {
        if !self.r.running() {
            return;
        }
        for d in 0..4 {
            if self.input.pressed[d] && self.slide(d) {
                self.moves += 1;
                self.r.cues.push(Fx::Tick);
                if self.cells == SOLVED {
                    self.r.score[0] = 1000_u32.saturating_sub(self.moves).max(1);
                    finish(&mut self.r, 0);
                }
                break;
            }
        }
    }
    fn render(&self) {
        bg(0x192f43);
        caption(&format!("# {}", self.moves), 30., 65., 30., WHITE);
        for (i, &v) in self.cells.iter().enumerate() {
            let (x, y) = (264. + (i % 4) as f32 * 108., 53. + (i / 4) as f32 * 108.);
            draw_rectangle(
                x,
                y,
                102.,
                102.,
                if v == 0 {
                    Color::from_hex(0x294457)
                } else if usize::from(v) == i + 1 {
                    Color::from_hex(0xbbe3ae)
                } else {
                    Color::from_hex(0x9acee3)
                },
            );
            if v != 0 {
                caption(&v.to_string(), x + 20., y + 65., 48., BLACK);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_slide_finishes() {
        let mut g = Tiles {
            r: Round::new(1),
            cells: SOLVED,
            blank: 15,
            ..Default::default()
        };
        g.slide(2);
        g.input.pressed[3] = true;
        g.update(0.);
        assert_eq!(g.r.result, ResultState::Winner(0));
    }
}
