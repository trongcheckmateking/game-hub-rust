use crate::core::{game::*,input::PlayerFrame};
use macroquad::prelude::*;
use std::collections::{HashSet,VecDeque};
pub const TITLE:&str="Snake Duel";
type Cell=(i32,i32);
const W:i32=28;const H:i32=16;
#[derive(Default)]
pub struct Snake{round:Round,bodies:[VecDeque<Cell>;2],dir:[Cell;2],queue:[VecDeque<Cell>;2],food:Cell,seed:u64,timer:f32,elapsed:f32}
impl Snake{
    fn spawn(&mut self){
        let cells:HashSet<_>=self.bodies.iter().flatten().copied().collect();
        let free:Vec<_>=(0..H).flat_map(|y|(0..W).map(move|x|(x,y))).filter(|p|!cells.contains(p)).collect();
        if free.is_empty(){self.round.timed_finish(&[0,1]);}else{self.food=free[next_random(&mut self.seed)%free.len()];}
    }
    fn step(&mut self){
        let heads:[Cell;2]=std::array::from_fn(|i|{if let Some(d)=self.queue[i].pop_front(){self.dir[i]=d;}
            let (x,y)=self.bodies[i][0];(x+self.dir[i].0,y+self.dir[i].1)});
        let mut solid=HashSet::new();
        for(i,b) in self.bodies.iter().enumerate(){solid.extend(b.iter().take(b.len()-usize::from(heads[i]!=self.food)).copied());}
        let dead=heads.map(|(x,y)|heads[0]==heads[1]||!(0..W).contains(&x)||!(0..H).contains(&y)||solid.contains(&(x,y)));
        if dead.iter().any(|v|*v){self.round.cues.push(Fx::Lose);
            if let Some(i)=dead.iter().position(|v|!*v){self.round.score[i]+=5;self.round.result=ResultState::Winner(i);}
            else{self.round.result=ResultState::Draw;}return;
        }
        for(i,p) in heads.iter().enumerate(){self.bodies[i].push_front(*p);
            if *p==self.food{self.round.score[i]+=2;self.round.cues.push(Fx::Pickup);}else{self.bodies[i].pop_back();}}
        if heads.contains(&self.food){self.spawn();}
    }
}
impl MiniGame for Snake{
    fn init(&mut self,n:usize,c:Config){assert_eq!(n,2);*self=Self{round:Round::new(n),seed:c.seed,
        bodies:[[(6,5),(5,5),(4,5)].into(),[(21,10),(22,10),(23,10)].into()],dir:[(1,0),(-1,0)],..Default::default()};self.spawn();}
    fn round(&self)->&Round{&self.round}
    fn round_mut(&mut self)->&mut Round{&mut self.round}
    fn handle_input(&mut self,f:&[PlayerFrame]){
        for(i,frame) in f.iter().take(2).enumerate(){for(a,d) in DIRECTIONS.iter().enumerate(){
            let prev=self.queue[i].back().copied().unwrap_or(self.dir[i]);
            if frame.pressed[a]&&self.queue[i].len()<2&&prev.0*d.0+prev.1*d.1==0{self.queue[i].push_back(*d);}
        }}
    }
    fn update(&mut self,dt:f32){if !self.round.running(){return;}self.elapsed+=dt;self.timer+=dt;
        if self.elapsed>=90.{self.round.timed_finish(&[0,1]);return;}
        let tick=(0.16-self.round.score.iter().sum::<u32>() as f32*0.001).max(0.10);
        if self.timer>=tick{self.timer-=tick;self.step();}
    }
    fn render(&self){
        draw_rectangle(0.,0.,960.,540.,Color::from_hex(0x182c29));
        let pos=|(x,y):Cell|vec2(88.+x as f32*28.,45.+y as f32*28.);
        for y in 0..H{for x in 0..W{let p=pos((x,y));draw_rectangle(p.x,p.y,28.,28.,Color::from_hex(if(x+y)%2==0{0x203a33}else{0x254139}));}}
        for(i,body) in self.bodies.iter().enumerate(){for(j,&c) in body.iter().enumerate(){let p=pos(c);
            draw_rectangle(p.x+1.,p.y+1.,26.,26.,super::color(i));
            if j==0{let(dx,dy)=self.dir[i];for sign in [-1,1]{draw_circle(p.x+14.+(dx*7+dy*5*sign) as f32,p.y+14.+(dy*7+dx*5*sign) as f32,2.5,BLACK);}}
        }}
        let p=pos(self.food);draw_poly(p.x+14.,p.y+14.,4,12.,0.,Color::from_hex(0xff9a7d));
        draw_text(&format!("{}s",(90.-self.elapsed).max(0.).ceil()),815.,32.,22.,WHITE);
    }
}
#[cfg(test)]mod tests{use super::*;#[test]fn equal_head_collision_is_draw(){let mut g=Snake::default();g.init(2,Config{seed:3});
g.bodies=[[(5,5),(4,5)].into(),[(7,5),(8,5)].into()];g.step();assert_eq!(g.round.result,ResultState::Draw);}}
