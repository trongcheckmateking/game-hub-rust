use crate::core::{game::*,input::PlayerFrame};
use macroquad::prelude::*;
pub const TITLE:&str="Pong";
#[derive(Default)]
pub struct Pong{round:Round,pads:[f32;2],move_y:[f32;2],ball:Vec2,velocity:Vec2,delay:f32,seed:u64}
impl Pong{
    fn serve(&mut self,dir:f32){
        self.ball=vec2(480.,270.);let angle=(next_random(&mut self.seed)%75) as f32/100.-0.37;
        self.velocity=vec2(angle.cos()*dir,angle.sin())*345.;self.delay=0.85;
    }
}
impl MiniGame for Pong{
    fn init(&mut self,n:usize,c:Config){assert_eq!(n,2);*self=Self{round:Round::new(n),pads:[270.;2],seed:c.seed,..Default::default()};self.serve(1.);}
    fn round(&self)->&Round{&self.round}
    fn round_mut(&mut self)->&mut Round{&mut self.round}
    fn handle_input(&mut self,f:&[PlayerFrame]){for(i,m) in self.move_y.iter_mut().enumerate(){*m=(f[i].held[1] as i8-f[i].held[0] as i8) as f32;}}
    fn update(&mut self,dt:f32){
        if !self.round.running(){return;}
        for(i,y) in self.pads.iter_mut().enumerate(){*y=(*y+self.move_y[i]*440.*dt).clamp(73.,467.);}
        if self.delay>0.{self.delay-=dt;return;}
        self.ball+=self.velocity*dt;
        if (self.ball.y<25. && self.velocity.y<0.)||(self.ball.y>515. && self.velocity.y>0.){
            self.ball.y=self.ball.y.clamp(25.,515.);self.velocity.y *= -1.;self.round.cues.push(Fx::Tick);
        }
        let i=usize::from(self.velocity.x>0.);let x=[42.,906.][i];
        if overlaps((self.ball.x-7.,self.ball.y-7.,14.,14.),(x,self.pads[i]-54.,12.,108.)){
            let angle=((self.ball.y-self.pads[i])/54.).clamp(-1.,1.)*1.04;
            let speed=(self.velocity.length()+30.).min(790.);
            self.velocity=vec2(angle.cos()*[1.,-1.][i],angle.sin())*speed;
            self.ball.x=if i==0{x+19.}else{x-7.};self.round.cues.push(Fx::Hit);
        }
        if self.ball.x < -8. || self.ball.x>968.{
            let p=usize::from(self.ball.x<0.);self.round.score[p]+=1;self.round.cues.push(Fx::Pickup);
            if self.round.score[p]==7{self.round.result=ResultState::Winner(p);}else{self.serve([1.,-1.][p]);}
        }
    }
    fn render(&self){
        draw_rectangle(0.,0.,960.,540.,Color::from_hex(0x172c40));let line=Color::from_hex(0x426078);
        draw_rectangle_lines(22.,18.,916.,504.,3.,line);draw_circle_lines(480.,270.,80.,2.,line);
        for j in 0..24{draw_rectangle(479.,19.+j as f32*21.,2.,9.,line);}
        for(i,y) in self.pads.iter().enumerate(){draw_rectangle([42.,906.][i],*y-54.,12.,108.,super::color(i));}
        draw_circle(self.ball.x,self.ball.y,8.,Color::from_hex(0xffecb8));
    }
}
#[cfg(test)]
mod tests{use super::*;#[test]fn seventh_point_finishes_match(){let mut g=Pong::default();g.init(2,Config{seed:1});g.round.score[0]=6;g.delay=0.;g.ball=vec2(980.,270.);g.update(0.01);assert_eq!(g.round.result,ResultState::Winner(0));}}
