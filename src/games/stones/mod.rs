use crate::core::{game::*,input::PlayerFrame};
use super::table::*;use macroquad::prelude::*;
#[derive(Default)]pub struct Stones{r:Round,left:usize,take:usize,actor:usize,seats:usize,input:PlayerFrame,delay:f32,seed:u64}
impl Stones{
    fn remove(&mut self){let n=self.take.min(self.left);self.left-=n;if self.actor<self.r.score.len(){self.r.score[self.actor]+=n as u32;}
        self.r.cues.push(Fx::Pickup);if self.left==0{finish(&mut self.r,self.actor);}else{self.actor=(self.actor+1)%self.seats;self.take=1;self.delay=0.45;}}
}
impl MiniGame for Stones{
    fn init(&mut self,n:usize,c:Config){*self=Self{r:Round::new(n),left:21,take:1,seats:n.max(2),seed:c.seed,..Self::default()};}
    fn round(&self)->&Round{&self.r}fn round_mut(&mut self)->&mut Round{&mut self.r}
    fn handle_input(&mut self,f:&[PlayerFrame]){self.input=f.get(self.actor).copied().unwrap_or_default();}
    fn update(&mut self,dt:f32){if !self.r.running(){return;}if self.actor>=self.r.score.len(){self.delay-=dt;if self.delay<=0.{self.take=self.left%4;if self.take==0{self.take=next_random(&mut self.seed)%3+1;}self.remove();}return;}
        let max=self.left.min(3);if self.input.pressed[2]{self.take=(self.take+max-2)%max+1;}if self.input.pressed[3]{self.take=self.take%max+1;}if self.input.pressed[4]{self.remove();}}
    fn render(&self){bg(0x31302c);turn(self.actor,self.r.score.len()==1);caption(&format!("{}",self.left),430.,100.,69.,GOLD);
        for i in 0..self.left{let x=202.+(i%7)as f32*91.;let y=180.+(i/7)as f32*85.;circle(x,y,28.,if i>=self.left-self.take.min(self.left){super::color(self.actor)}else{Color::from_hex(0xb7b09d)});}
        caption(&format!("<     {}     >        A",self.take),324.,468.,40.,WHITE);
    }
}
#[cfg(test)]mod tests{use super::*;#[test]fn last_stone_wins_for_third_player(){let mut g=Stones::default();g.init(3,Config{seed:1});g.actor=2;g.left=1;g.remove();assert_eq!(g.r.result,ResultState::Winner(2));}}
