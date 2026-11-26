use super::table::*;
use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;
const HILLS: [(f32, f32); 4] = [(480., 270.), (240., 160.), (720., 260.), (444., 408.)];
#[derive(Default)]
pub struct Hill {
    r: Round,
    pos: Vec<Vec2>,
    frames: Vec<PlayerFrame>,
    time: f32,
    credit: Vec<f32>,
    boost: Vec<f32>,
    cool: Vec<f32>,
}
impl Hill {
    fn center(&self) -> Vec2 {
        let p = HILLS[(self.time / 6.) as usize % 4];
        vec2(p.0, p.1)
    }
}
impl MiniGame for Hill {
    fn init(&mut self, n: usize, _: Config) {
        let starts = [
            vec2(170., 270.),
            vec2(790., 270.),
            vec2(480., 75.),
            vec2(480., 465.),
        ];
        *self = Self {
            r: Round::new(n),
            pos: starts[..n].into(),
            frames: vec![PlayerFrame::default(); n],
            credit: vec![0.; n],
            boost: vec![0.; n],
            cool: vec![0.; n],
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
        self.frames.clone_from_slice(f);
    }
    fn update(&mut self, dt: f32) {
        if !self.r.running() {
            return;
        }
        self.time += dt;
        for (i, p) in self.pos.iter_mut().enumerate() {
            let f = self.frames[i];
            self.cool[i] = (self.cool[i] - dt).max(0.);
            self.boost[i] = (self.boost[i] - dt).max(0.);
            if f.pressed[4] && self.cool[i] == 0. {
                self.boost[i] = 0.22;
                self.cool[i] = 1.65;
            }
            let v = vec2(
                (f.held[3] as i8 - f.held[2] as i8) as f32,
                (f.held[1] as i8 - f.held[0] as i8) as f32,
            )
            .normalize_or_zero();
            *p = (*p + v * dt * if self.boost[i] > 0. { 530. } else { 210. })
                .clamp(vec2(23., 23.), vec2(937., 517.));
        }
        let h = self.center();
        let mut inside =
            (0..self.pos.len()).filter(|&i| self.pos[i].distance_squared(h) <= 66. * 66.);
        if let (Some(i), None) = (inside.next(), inside.next()) {
            self.credit[i] += dt;
            while self.credit[i] >= 0.5 {
                self.credit[i] -= 0.5;
                self.r.score[i] += 1;
            }
        }
        if self.time >= 36. {
            self.r
                .timed_finish(&(0..self.pos.len()).collect::<Vec<_>>());
        }
    }
    fn render(&self) {
        bg(0x162e37);
        let h = self.center();
        for x in (0..960).step_by(48) {
            draw_line(x as f32, 0., x as f32, 540., 1., DARKGRAY);
        }
        for y in (0..540).step_by(48) {
            draw_line(0., y as f32, 960., y as f32, 1., DARKGRAY);
        }
        draw_circle(h.x, h.y, 66., Color::from_hex(0x617153));
        draw_circle_lines(h.x, h.y, 66., 4., GOLD);
        for (i, p) in self.pos.iter().enumerate() {
            circle(p.x, p.y, 18., super::color(i));
            caption(&(i + 1).to_string(), p.x - 6., p.y + 7., 22., BLACK);
            if self.cool[i] == 0. {
                draw_circle_lines(p.x, p.y, 23., 2., WHITE);
            }
        }
        caption(
            &format!("{}s", (36. - self.time).ceil().max(0.)),
            840.,
            50.,
            30.,
            WHITE,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contested_hill_does_not_award_points() {
        let mut g = Hill::default();
        g.init(3, Config { seed: 1 });
        g.pos[0] = g.center();
        g.pos[1] = g.center();
        g.update(0.6);
        assert_eq!(g.r.score, [0, 0, 0]);
        g.pos[1] = Vec2::ZERO;
        g.update(0.6);
        assert_eq!(g.r.score, [1, 0, 0]);
    }
}
