//! A four-seat arena with switchable one-axis pulses and chain reactions.
use crate::core::{game::*,input::PlayerFrame};
use macroquad::prelude::*;
use std::collections::BTreeMap;
pub const TITLE:&str="Fuse Yard";
type Cell=(i32,i32);
#[derive(Clone,Copy,Default)]
struct Runner{pos:Cell,alive:bool,vertical:bool,cool:f32}
#[derive(Clone,Copy)]
struct Core{pos:Cell,owner:usize,vertical:bool,fuse:f32}
#[derive(Default)]
pub struct FuseYard{round:Round,players:[Runner;4],tiles:[[u8;15];9],cores:Vec<Core>,pulse:BTreeMap<Cell,f32>,input:[PlayerFrame;4],elapsed:f32}
impl FuseYard{
    fn fire(&mut self,index:usize){
        let b=self.cores.remove(index);self.pulse.insert(b.pos,0.43);self.round.cues.push(Fx::Explode);
        for sign in [-1,1]{for k in 1..=3{
            let(x,y)=if b.vertical{(b.pos.0,b.pos.1+sign*k)}else{(b.pos.0+sign*k,b.pos.1)};
            if self.tiles[y as usize][x as usize]==2{break;}self.pulse.insert((x,y),0.43);
            if self.tiles[y as usize][x as usize]==1{self.tiles[y as usize][x as usize]=0;self.round.score[b.owner]+=1;break;}
            if let Some(i)=self.cores.iter().position(|o|o.pos==(x,y)){self.fire(i);}
        }}
    }
    fn eliminate(&mut self){for p in &mut self.players{if p.alive&&self.pulse.contains_key(&p.pos){p.alive=false;self.round.cues.push(Fx::Lose);}}}
}
impl MiniGame for FuseYard{
    fn init(&mut self,n:usize,_:Config){assert_eq!(n,4);*self=Self{round:Round::new(n),..Default::default()};
        for(i,pos) in [(1,1),(13,7),(13,1),(1,7)].into_iter().enumerate(){self.players[i]=Runner{pos,alive:true,vertical:i%2==1,cool:0.};}
        for y in 0usize..9{for x in 0usize..15{let xx=x.min(14-x);let yy=y.min(8-y);
            self.tiles[y][x]=if xx==0||yy==0||(yy==3&&[3,7].contains(&xx))||(xx==5&&yy==2){2}
            else if (xx+yy)%3==0&&xx+yy>3{1}else{0};
        }}
    }
    fn round(&self)->&Round{&self.round}
    fn round_mut(&mut self)->&mut Round{&mut self.round}
    fn handle_input(&mut self,f:&[PlayerFrame]){self.input.copy_from_slice(&f[..4]);}
    fn update(&mut self,dt:f32){
        if !self.round.running(){return;}self.elapsed+=dt;
        self.pulse.retain(|_,t|{*t-=dt;*t>0.});self.eliminate();
        for i in 0..4{let mut p=self.players[i];if !p.alive{continue;}let f=self.input[i];p.cool=(p.cool-dt).max(0.);
            if f.pressed[5]{p.vertical=!p.vertical;}
            if f.pressed[4]&&!self.cores.iter().any(|o|o.owner==i||o.pos==p.pos){
                self.cores.push(Core{pos:p.pos,owner:i,vertical:p.vertical,fuse:1.6});self.round.cues.push(Fx::Tick);}
            let dir=(0..4).find(|&d|f.pressed[d]).or_else(||(0..4).find(|&d|f.held[d]));
            if let Some(d)=dir.filter(|_|p.cool==0.){let (dx,dy)=DIRECTIONS[d];let cell=(p.pos.0+dx,p.pos.1+dy);
                if self.tiles[cell.1 as usize][cell.0 as usize]==0&&!self.cores.iter().any(|b|b.pos==cell)&&!self.players.iter().any(|r|r.alive&&r.pos==cell){
                    p.pos=cell;p.cool=0.125;
                }}
            self.players[i]=p;
        }
        for b in &mut self.cores{b.fuse-=dt;}
        while let Some(i)=self.cores.iter().position(|b|b.fuse<=0.||self.pulse.contains_key(&b.pos)){self.fire(i);}
        self.eliminate();let live:Vec<_>=(0..4).filter(|&i|self.players[i].alive).collect();
        match live.as_slice(){[]=>self.round.result=ResultState::Draw,[i]=>{self.round.score[*i]+=5;self.round.result=ResultState::Winner(*i);},
            _ if self.elapsed>=100.=>self.round.timed_finish(&live),_=>{}}
    }
    fn render(&self){
        draw_rectangle(0.,0.,960.,540.,Color::from_hex(0x282432));let pos=|(x,y):Cell|vec2(105.+x as f32*50.,45.+y as f32*50.);
        for (y,row) in self.tiles.iter().enumerate(){for(x,t) in row.iter().enumerate(){let c=(x as i32,y as i32);let p=pos(c);
            draw_rectangle(p.x,p.y,48.,48.,Color::from_hex([0x393747,0xdba079,0x666477][*t as usize]));
            match t{1=>{for h in [12.,30.]{draw_rectangle(p.x+5.,p.y+h,38.,4.,Color::from_hex(0xb77a59));}},2=>{draw_rectangle(p.x+3.,p.y+3.,41.,4.,GRAY);},_=>{}}
            if self.pulse.contains_key(&c){draw_rectangle(p.x,p.y,48.,48.,ORANGE);draw_rectangle(p.x+7.,p.y+7.,34.,34.,YELLOW);}
        }}
        for b in &self.cores{let p=pos(b.pos)+vec2(24.,24.);draw_circle(p.x,p.y,18.,if(b.fuse*10.) as i32%2==0{BLACK}else{MAROON});
            if b.vertical{draw_line(p.x,p.y-12.,p.x,p.y+12.,5.,YELLOW);}else{draw_line(p.x-12.,p.y,p.x+12.,p.y,5.,YELLOW);}}
        for(i,r) in self.players.iter().enumerate(){if r.alive{let p=pos(r.pos);draw_rectangle(p.x+8.,p.y+7.,32.,35.,super::color(i));
            draw_text(&(i+1).to_string(),p.x+17.,p.y+29.,23.,BLACK);
            let (w,h)=if r.vertical{(3.,11.)}else{(12.,3.)};draw_rectangle(p.x+24.-w/2.,p.y+36.-h/2.,w,h,BLACK);
        }}
        draw_text(&format!("{}s",(100.-self.elapsed).max(0.).ceil()),805.,32.,22.,WHITE);
    }
}
#[cfg(test)]mod tests{use super::*;#[test]fn core_can_trigger_its_neighbor(){
let mut g=FuseYard::default();g.init(4,Config{seed:1});g.cores=vec![Core{pos:(1,1),owner:0,vertical:false,fuse:0.},Core{pos:(2,1),owner:1,vertical:true,fuse:1.}];
g.fire(0);assert!(g.cores.is_empty());assert!(g.pulse.contains_key(&(2,2)));}}
