use super::table::*;
use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;
#[derive(Default)]
pub struct Beacon {
    r: Round,
    seed: u64,
    heat: u8,
    time: f32,
    signal: f32,
    seen: Vec<bool>,
    keys: Vec<bool>,
    gap: f32,
}
impl Beacon {
    fn prepare(&mut self) {
        self.time = 0.;
        self.signal = 1.25 + (next_random(&mut self.seed) % 160) as f32 / 100.;
        self.seen.fill(false);
    }
    fn winner(&mut self) {
        if self.r.score.len() == 1 && self.r.score[0] == 0 {
            self.r.result = ResultState::Defeat;
        } else {
            self.r
                .timed_finish(&(0..self.r.score.len()).collect::<Vec<_>>());
        }
    }
}
impl MiniGame for Beacon {
    fn init(&mut self, n: usize, c: Config) {
        *self = Self {
            r: Round::new(n),
            seed: c.seed,
            heat: 1,
            seen: vec![false; n],
            keys: vec![false; n],
            ..Self::default()
        };
        self.prepare();
    }
    fn round(&self) -> &Round {
        &self.r
    }
    fn round_mut(&mut self) -> &mut Round {
        &mut self.r
    }
    fn handle_input(&mut self, f: &[PlayerFrame]) {
        for (i, p) in f.iter().enumerate() {
            self.keys[i] = p.pressed[4];
        }
    }
    fn update(&mut self, dt: f32) {
        if !self.r.running() {
            return;
        }
        if self.gap > 0. {
            self.gap -= dt;
            if self.gap <= 0. {
                if self.heat == 5 {
                    self.winner();
                } else {
                    self.heat += 1;
                    self.prepare();
                }
            }
            return;
        }
        let prev = self.time;
        self.time += dt;
        if prev < self.signal && self.time >= self.signal {
            self.r.cues.push(Fx::Tick);
        }
        for (i, &tap) in self.keys.iter().enumerate() {
            if tap && !self.seen[i] {
                self.seen[i] = true;
                let late = self.time - self.signal;
                if (0.0..=1.2).contains(&late) {
                    self.r.score[i] += (1000. - late * 750.).round().max(1.) as u32;
                    self.r.cues.push(Fx::Pickup);
                } else {
                    self.r.cues.push(Fx::Lose);
                }
            }
        }
        if self.time >= self.signal + 1.2 || self.seen.iter().all(|v| *v) {
            self.gap = 0.7;
        }
    }
    fn render(&self) {
        bg(0x22243a);
        let go = self.time >= self.signal && self.gap <= 0.;
        caption(&format!("{} / 5", self.heat), 40., 60., 32., LIGHTGRAY);
        draw_circle_lines(480., 249., 106., 4., GRAY);
        circle(
            480.,
            249.,
            87.,
            if self.gap > 0. {
                GRAY
            } else if go {
                LIME
            } else {
                Color::from_hex(0xc45b67)
            },
        );
        caption(
            if self.gap > 0. {
                "-"
            } else if go {
                "A!"
            } else {
                "..."
            },
            449.,
            269.,
            54.,
            BLACK,
        );
        for (i, v) in self.seen.iter().enumerate() {
            let x = 140. + i as f32 * 202.;
            caption(
                &format!("P{}   {}", i + 1, self.r.score[i]),
                x,
                448.,
                28.,
                super::color(i),
            );
            if *v {
                draw_line(x, 461., x + 91., 461., 4., WHITE);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn early_press_locks_that_seat_only() {
        let mut g = Beacon::default();
        g.init(3, Config { seed: 1 });
        g.keys[0] = true;
        g.update(0.01);
        g.keys = [false, true, false].into();
        g.time = g.signal;
        g.update(0.01);
        assert_eq!(g.r.score[0], 0);
        assert!(g.r.score[1] > 0);
    }
}
