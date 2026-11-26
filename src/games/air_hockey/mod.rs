//! Fixed-step two-player air hockey. Physics uses rink units, independent of the display.
use crate::core::{game::*, input::PlayerFrame};
use macroquad::prelude::*;

const CENTER: Vec2 = Vec2::new(480., 270.);
const RAIL: f32 = 36.;
const GOAL: f32 = 80.;
const PUCK: f32 = 13.;
const MALLET: f32 = 27.;
const MAX_SPEED: f32 = 820.;

#[derive(Default)]
pub struct AirHockey {
    round: Round,
    mallets: [Vec2; 2],
    old_mallets: [Vec2; 2],
    input: [Vec2; 2],
    puck: Vec2,
    velocity: Vec2,
    trail: [Vec2; 8],
    countdown: f32,
    impact: f32,
    seed: u64,
}

impl AirHockey {
    fn serve(&mut self, direction: f32) {
        self.puck = CENTER;
        self.trail.fill(CENTER);
        self.mallets = [vec2(193., 270.), vec2(767., 270.)];
        self.old_mallets = self.mallets;
        self.velocity = vec2(direction * 245., (next_random(&mut self.seed) % 121) as f32 - 60.);
        self.countdown = 0.65;
        self.impact = 0.;
    }

    fn score(&mut self, player: usize) {
        self.round.score[player] += 1;
        self.round.cues.push(Fx::Pickup);
        if self.round.score[player] == 5 {
            self.round.result = ResultState::Winner(player);
        } else {
            self.serve(if player == 0 { 1. } else { -1. });
        }
    }

    fn hit(&mut self, center: Vec2, radius: f32, surface_velocity: Vec2) {
        let delta = self.puck - center;
        let min_distance = PUCK + radius;
        if delta.length_squared() >= min_distance * min_distance {
            return;
        }
        let normal = delta.try_normalize().unwrap_or(Vec2::X);
        self.puck = center + normal * min_distance;
        let incoming = (self.velocity - surface_velocity).dot(normal);
        if incoming < 0. {
            self.velocity = (self.velocity - 1.92 * incoming * normal).clamp_length_max(MAX_SPEED);
            self.round.cues.push(Fx::Hit);
            self.impact = 1.;
        }
    }

    fn advance(&mut self, dt: f32) {
        let previous = self.mallets;
        for (i, pos) in self.mallets.iter_mut().enumerate() {
            let (left, right) = if i == 0 { (RAIL, CENTER.x) } else { (CENTER.x, 960. - RAIL) };
            *pos = (*pos + self.input[i] * (410. * dt)).clamp(
                vec2(left + MALLET, RAIL + MALLET), vec2(right - MALLET, 540. - RAIL - MALLET),
            );
        }
        if self.countdown > 0. {
            self.countdown = (self.countdown - dt).max(0.);
            return;
        }
        self.puck += self.velocity * dt;
        self.velocity *= 1. - dt * 0.14;
        for (i, &old) in previous.iter().enumerate() {
            self.hit(self.mallets[i], MALLET, (self.mallets[i] - old) / dt);
        }
        let y = self.puck.y.clamp(RAIL + PUCK, 540. - RAIL - PUCK);
        if y != self.puck.y {
            self.velocity.y = self.velocity.y.abs().copysign(y - self.puck.y);
            self.puck.y = y;
        }
        for x in [RAIL, 960. - RAIL] {
            for y in [CENTER.y - GOAL, CENTER.y + GOAL] {
                self.hit(vec2(x, y), 4., Vec2::ZERO);
            }
        }
        if (self.puck.y - CENTER.y).abs() >= GOAL {
            let x = self.puck.x.clamp(RAIL + PUCK, 960. - RAIL - PUCK);
            if x != self.puck.x {
                self.velocity.x = self.velocity.x.abs().copysign(x - self.puck.x);
                self.puck.x = x;
            }
        }
        if self.puck.x < RAIL - PUCK { self.score(1); }
        else if self.puck.x > 960. - RAIL + PUCK { self.score(0); }
    }

    fn paint(&self, alpha: f32) {
        let c = Color::from_hex;
        draw_rectangle(0., 0., 960., 540., c(0x10272d));
        let line = c(0x38616a);
        draw_rectangle(RAIL, RAIL, 960.-2.*RAIL, 540.-2.*RAIL, c(0x173d43));
        draw_line(480., RAIL, 480., 540.-RAIL, 2., line);
        draw_circle_lines(480., 270., 78., 3., line);
        draw_circle(480., 270., 5., line);
        for x in [RAIL, 960.-RAIL] {
            for (from,to) in [(RAIL,270.-GOAL),(270.+GOAL,540.-RAIL)] {
                draw_line(x, from, x, to, 7., c(0x89b5ae));
                draw_circle(x,from,3.5,c(0x89b5ae));
                draw_circle(x,to,3.5,c(0x89b5ae));
            }
        }
        for y in [RAIL, 540.-RAIL] { draw_line(RAIL,y,960.-RAIL,y,6.,c(0x89b5ae)); }
        let puck = self.trail[0].lerp(self.puck, alpha);
        for i in (1..self.trail.len()).rev() {
            let t=1.-i as f32/self.trail.len() as f32;
            let p=self.trail[i].lerp(self.trail[i-1],alpha);
            draw_circle(p.x,p.y,PUCK*(0.38+0.55*t),Color::new(0.60,0.95,0.89,t*0.19));
        }
        for (i, &current) in self.mallets.iter().enumerate() {
            let p=self.old_mallets[i].lerp(current,alpha);
            let tint=[c(0xabebdb),c(0xff9c86)][i];
            draw_circle(p.x+2.,p.y+5.,MALLET+1.,c(0x102e33));
            draw_circle(p.x,p.y,MALLET,tint);
            draw_circle(p.x,p.y+1.,MALLET*0.63,c(0x376268));
            draw_circle(p.x,p.y-3.,MALLET*0.55,tint);
            draw_circle(p.x-5.,p.y-8.,4.,Color::new(1.,1.,1.,0.45));
        }
        draw_circle(puck.x+1.,puck.y+3.,PUCK,c(0x102e33));
        draw_circle(puck.x,puck.y,PUCK,c(0xfaf4d7));
        draw_circle(puck.x,puck.y,PUCK-4.,c(0xddc89a));
        if self.impact > 0. {
            draw_circle_lines(puck.x,puck.y,PUCK+4.+(1.-self.impact)*17.,2.,Color::new(0.82,1.,0.9,self.impact*0.65));
        }
    }
}
impl MiniGame for AirHockey {
    fn init(&mut self,n:usize,config:Config) {
        assert_eq!(n,2);
        *self=Self{round:Round::new(n),seed:config.seed,..Self::default()};
        self.serve(if config.seed & 1 == 0 {1.} else {-1.});
    }
    fn round(&self)->&Round { &self.round }
    fn round_mut(&mut self)->&mut Round { &mut self.round }
    fn handle_input(&mut self,players:&[PlayerFrame]) {
        for (v,p) in self.input.iter_mut().zip(players) {
            *v=vec2((p.held[3] as i8-p.held[2] as i8) as f32,(p.held[1] as i8-p.held[0] as i8) as f32).normalize_or_zero();
        }
    }
    fn update(&mut self,dt:f32) {
        if !self.round.running() || dt <= 0. { return; }
        self.old_mallets=self.mallets;
        self.trail.rotate_right(1);
        self.trail[0]=self.puck;
        self.impact=(self.impact-dt*4.).max(0.);
        for _ in 0..2 {
            self.advance(dt*0.5);
            if !self.round.running() { break; }
        }
    }
    fn render(&self) { self.paint(1.); }
    fn render_interpolated(&self, alpha:f32) { self.paint(alpha); }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rails_bounce_and_only_open_goals_score() {
        let mut g=AirHockey::default();
        g.init(2,Config{seed:10});
        g.countdown=0.;
        g.puck=vec2(50.,90.);
        g.velocity=vec2(-600.,0.);
        g.update(1./60.);
        assert!(g.velocity.x>0.);
        assert_eq!(g.round.score,[0,0]);
        g.round.score[1]=4;
        g.puck=vec2(25.,270.);
        g.velocity=vec2(-600.,0.);
        g.update(1./60.);
        assert_eq!(g.round.result,ResultState::Winner(1));
    }
    #[test]
    fn moving_mallet_transfers_momentum_to_puck() {
        let mut g=AirHockey::default();
        g.init(2,Config{seed:0});
        g.countdown=0.;
        g.puck=g.mallets[0]+vec2(MALLET+PUCK+1.,0.);
        g.velocity=Vec2::ZERO;
        g.input[0]=Vec2::X;
        g.update(1./60.);
        assert!(g.velocity.x>200. && g.velocity.length()<=MAX_SPEED);
    }
}
