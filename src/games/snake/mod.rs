use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;
use std::collections::VecDeque;
pub const TITLE: &str = "Snake Duel";
type Cell = (i32, i32);
const W: i32 = 28;
const H: i32 = 16;
#[derive(Default)]
pub struct Snake {
    round: Round,
    bodies: [VecDeque<Cell>; 2],
    dir: [Cell; 2],
    queue: [VecDeque<Cell>; 2],
    food: Cell,
    seed: u64,
    timer: f32,
    elapsed: f32,
    last_tail: [Cell; 2],
    advanced: bool,
}
impl Snake {
    fn interval(&self) -> f32 {
        (0.16 - self.round.score.iter().sum::<u32>() as f32 * 0.001).max(0.10)
    }
    fn cell_index((x, y): Cell) -> usize {
        (y * W + x) as usize
    }
    fn spawn(&mut self) {
        let mut occupied = [false; (W * H) as usize];
        for &cell in self.bodies.iter().flatten() {
            occupied[Self::cell_index(cell)] = true;
        }
        let free = occupied.iter().filter(|&&taken| !taken).count();
        if free == 0 {
            self.round.timed_finish(&[0, 1]);
            return;
        }
        let choice = next_random(&mut self.seed) % free;
        let index = occupied
            .iter()
            .enumerate()
            .filter(|(_, used)| !**used)
            .nth(choice)
            .expect("free cell counted above")
            .0 as i32;
        self.food = (index % W, index / W);
    }
    fn step(&mut self) {
        let heads: [Cell; 2] = std::array::from_fn(|i| {
            if let Some(d) = self.queue[i].pop_front() {
                self.dir[i] = d;
            }
            let (x, y) = self.bodies[i][0];
            (x + self.dir[i].0, y + self.dir[i].1)
        });
        let mut solid = [false; (W * H) as usize];
        for (i, b) in self.bodies.iter().enumerate() {
            for &cell in b.iter().take(b.len() - usize::from(heads[i] != self.food)) {
                solid[Self::cell_index(cell)] = true;
            }
        }
        let dead = heads.map(|(x, y)| {
            heads[0] == heads[1]
                || !(0..W).contains(&x)
                || !(0..H).contains(&y)
                || solid[Self::cell_index((x, y))]
        });
        if dead.iter().any(|v| *v) {
            self.round.cues.push(Fx::Lose);
            if let Some(i) = dead.iter().position(|v| !*v) {
                self.round.score[i] += 5;
                self.round.result = ResultState::Winner(i);
            } else {
                self.round.result = ResultState::Draw;
            }
            return;
        }
        self.advanced = true;
        for (i, p) in heads.iter().enumerate() {
            self.last_tail[i] = *self.bodies[i].back().unwrap();
            self.bodies[i].push_front(*p);
            if *p == self.food {
                self.round.score[i] += 2;
                self.round.cues.push(Fx::Pickup);
            } else {
                self.bodies[i].pop_back();
            }
        }
        if heads.contains(&self.food) {
            self.spawn();
        }
    }
}
impl MiniGame for Snake {
    fn init(&mut self, n: usize, c: Config) {
        assert_eq!(n, 2);
        *self = Self {
            round: Round::new(n),
            seed: c.seed,
            bodies: [
                [(6, 5), (5, 5), (4, 5)].into(),
                [(21, 10), (22, 10), (23, 10)].into(),
            ],
            dir: [(1, 0), (-1, 0)],
            ..Default::default()
        };
        self.spawn();
    }
    fn round(&self) -> &Round {
        &self.round
    }
    fn round_mut(&mut self) -> &mut Round {
        &mut self.round
    }
    fn handle_input(&mut self, f: &[PlayerFrame]) {
        for (i, frame) in f.iter().take(2).enumerate() {
            for (a, d) in DIRECTIONS.iter().enumerate() {
                let prev = self.queue[i].back().copied().unwrap_or(self.dir[i]);
                if frame.pressed[a] && self.queue[i].len() < 2 && prev.0 * d.0 + prev.1 * d.1 == 0 {
                    self.queue[i].push_back(*d);
                }
            }
        }
    }
    fn update(&mut self, dt: f32) {
        if !self.round.running() {
            return;
        }
        self.elapsed += dt;
        self.timer += dt;
        if self.elapsed >= 90. {
            self.round.timed_finish(&[0, 1]);
            return;
        }
        let tick = self.interval();
        if self.timer >= tick {
            self.timer -= tick;
            self.step();
        }
    }
    fn render(&self) {
        draw_rectangle(0., 0., 960., 540., Color::from_hex(0x182c29));
        let pos = |(x, y): Cell| vec2(102. + x as f32 * 28., 59. + y as f32 * 28.);
        draw_rectangle(
            88.,
            45.,
            W as f32 * 28.,
            H as f32 * 28.,
            Color::from_hex(0x234039),
        );
        let grid = Color::from_hex(0x2d4a41);
        for x in 1..W {
            let x = 88. + x as f32 * 28.;
            draw_line(x, 45., x, 493., 1., grid);
        }
        for y in 1..H {
            let y = 45. + y as f32 * 28.;
            draw_line(88., y, 872., y, 1., grid);
        }
        let t = (self.timer / self.interval()).clamp(0., 1.);
        for (i, body) in self.bodies.iter().enumerate() {
            let mut last: Option<Vec2> = None;
            for (j, &cell) in body.iter().enumerate().rev() {
                let from = body.get(j + 1).copied().unwrap_or(self.last_tail[i]);
                let p = if self.advanced {
                    pos(from).lerp(pos(cell), t)
                } else {
                    pos(cell)
                };
                let tint = super::color(i);
                if let Some(previous) = last {
                    draw_line(previous.x, previous.y, p.x, p.y, 20., tint);
                }
                draw_circle(p.x, p.y, if j == 0 { 12.5 } else { 10. }, tint);
                last = Some(p);
                if j == 0 {
                    let (dx, dy) = self.dir[i];
                    for sign in [-1, 1] {
                        let eye = p + vec2(
                            (dx * 5 + dy * 5 * sign) as f32,
                            (dy * 5 + dx * 5 * sign) as f32,
                        );
                        draw_circle(eye.x, eye.y, 3.1, Color::from_hex(0x193230));
                        draw_circle(eye.x - 0.6, eye.y - 0.7, 0.8, WHITE);
                    }
                }
            }
        }
        let p = pos(self.food);
        let radius = 9.5 + (self.elapsed * 4.).sin() * 0.8;
        draw_circle(p.x + 1., p.y + 2., radius, Color::from_hex(0x182f2d));
        draw_circle(p.x, p.y, radius, Color::from_hex(0xf88578));
        draw_circle(p.x - 3., p.y - 3., 2.3, Color::from_hex(0xffc8ac));
        draw_line(
            p.x,
            p.y - radius + 1.,
            p.x + 4.,
            p.y - radius - 3.,
            3.,
            Color::from_hex(0xb9dea1),
        );
        draw_text(
            &format!("{}s", (90. - self.elapsed).max(0.).ceil()),
            815.,
            32.,
            22.,
            WHITE,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_head_collision_is_draw() {
        let mut g = Snake::default();
        g.init(2, Config { seed: 3 });
        g.bodies = [[(5, 5), (4, 5)].into(), [(7, 5), (8, 5)].into()];
        g.step();
        assert_eq!(g.round.result, ResultState::Draw);
    }
}
