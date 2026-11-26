//! A short original race inspired by Ur. 0 = reserve, 15 = home.
use crate::core::{game::*,input::PlayerFrame};
use super::table::*;
use macroquad::prelude::*;
#[derive(Default)]
pub struct Ur { r:Round, pieces:[[u8;3];2], actor:usize, roll:u8, pick:usize, seed:u64, wait:f32, input:PlayerFrame }
impl Ur {
    fn legal(&self,i:usize)->bool{
        let pos=self.pieces[self.actor][i];let dest=pos+self.roll;
        self.roll>0 && pos<15 && dest<=15 &&
        (dest==15 || !self.pieces[self.actor].contains(&dest)) &&
        !(dest==8 && self.pieces[1-self.actor].contains(&dest))
    }
    fn choices(&self)->Vec<usize>{(0..3).filter(|&i|self.legal(i)).collect()}
    fn next(&mut self){self.actor=1-self.actor;self.roll=0;self.wait=0.38;}
    fn cast(&mut self){self.roll=(0..4).map(|_|(next_random(&mut self.seed)%2) as u8).sum();
        if let Some(&i)=self.choices().first(){self.pick=i;self.wait=0.35;}else{self.next();}
    }
    fn advance(&mut self,i:usize){
        if !self.legal(i){return;}
        let dest=self.pieces[self.actor][i]+self.roll;
        if (5..=12).contains(&dest){if let Some(other)=self.pieces[1-self.actor].iter_mut().find(|p|**p==dest){*other=0;self.r.cues.push(Fx::Hit);}}
        self.pieces[self.actor][i]=dest;
        if dest==15 {if self.actor<self.r.score.len(){self.r.score[self.actor]+=1;}self.r.cues.push(Fx::Pickup);}
        if self.pieces[self.actor].iter().all(|p|*p==15){finish(&mut self.r,self.actor);return;}
        if [4,8,14].contains(&dest){self.roll=0;self.wait=0.35;}else{self.next();}
    }
    fn cell(p:usize,n:u8)->(i32,i32){
        match n {1..=4=>(4-n as i32,if p==0{0}else{2}),5..=12=>(n as i32-5,1),13..=14=>(20-n as i32,if p==0{0}else{2}),_=>unreachable!()}
    }
}
impl MiniGame for Ur{
    fn init(&mut self,n:usize,c:Config){*self=Self{r:Round::new(n),seed:c.seed,..Self::default()};}
    fn round(&self)->&Round{&self.r}fn round_mut(&mut self)->&mut Round{&mut self.r}
    fn handle_input(&mut self,f:&[PlayerFrame]){self.input=f.get(self.actor).copied().unwrap_or_default();}
    fn update(&mut self,dt:f32){
        if !self.r.running(){return;}self.wait=(self.wait-dt).max(0.);let cpu=self.actor>=self.r.score.len();
        if cpu{if self.wait>0.{return;}if self.roll==0{self.cast();}else{let best=self.choices().into_iter().max_by_key(|&i|{
            let d=self.pieces[self.actor][i]+self.roll;(if d==15{60}else if (5..=12).contains(&d)&&self.pieces[0].contains(&d){45}else if [4,8,14].contains(&d){28}else{0})+d as i32
        });if let Some(i)=best{self.advance(i);}}return;}
        if self.roll>0{
            let v=self.choices();if !v.is_empty()&&self.input.pressed[..4].iter().any(|v|*v){let at=v.iter().position(|i|*i==self.pick).unwrap_or(0);self.pick=v[(at+1)%v.len()];}
        }
        if self.input.pressed[4]{if self.roll==0{self.cast();}else{self.advance(self.pick);}}
    }
    fn render(&self){bg(0x2c2527);turn(self.actor,self.r.score.len()==1);
        caption(&format!("A  {}   |   {} / 3",if self.roll==0{"ROLL".into()}else{format!("+{}",self.roll)},self.pieces[self.actor].iter().filter(|p|**p==15).count()),245.,72.,34.,GOLD);
        for p in 0..2 {for n in 1..=14 {if p==1&&(5..=12).contains(&n){continue;}let(x,y)=Self::cell(p,n);let x=112.+x as f32*92.;let y=126.+y as f32*91.;
            draw_rectangle(x,y,87.,85.,Color::from_hex(if [4,8,14].contains(&n){0xb99066}else{0x806b5a}));
            caption(&n.to_string(),x+6.,y+20.,20.,BLACK);if [4,8,14].contains(&n){draw_poly(x+44.,y+44.,8,21.,0.,GOLD);}
        }}
        for(p,pieces)in self.pieces.iter().enumerate(){for(i,&n)in pieces.iter().enumerate(){let(x,y)=if n==0{(85.+i as f32*55.,if p==0{97.}else{448.})}else if n==15{(693.+i as f32*46.,if p==0{97.}else{448.})}else{let(x,y)=Self::cell(p,n);(155.+x as f32*92.,168.+y as f32*91.)};
            circle(x,y,16.,super::color(p));if p==self.actor&&self.roll>0&&self.pick==i{draw_circle_lines(x,y,21.,3.,WHITE);}
        }}caption("1-4  /  5-12  /  13-14  /  HOME",255.,504.,24.,LIGHTGRAY);
    }
}
#[cfg(test)]mod tests{use super::*;#[test]fn protected_rosette_and_exact_finish(){let mut g=Ur::default();g.init(1,Config{seed:1});g.pieces=[[5,14,15],[8,2,3]];g.roll=3;assert!(!g.legal(0));assert!(!g.legal(1));g.roll=1;g.advance(1);assert_eq!(g.r.score,[1]);}}
