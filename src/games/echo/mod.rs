use crate::core::{game::*,input::PlayerFrame};
use super::table::*;use macroquad::prelude::*;
#[derive(Default)]pub struct Echo{r:Round,seed:u64,seq:Vec<usize>,actor:usize,level:usize,t:f32,step:usize,done:bool,gap:f32,input:PlayerFrame}
impl Echo{
    fn pattern(&mut self){self.seq=(0..self.level+2).map(|_|next_random(&mut self.seed)%4).collect();self.turn();}
    fn turn(&mut self){self.t=0.;self.step=0;self.done=false;self.gap=0.;}
    fn display_time(&self)->f32{0.65+self.seq.len()as f32*0.65}
}
impl MiniGame for Echo{
    fn init(&mut self,n:usize,c:Config){*self=Self{r:Round::new(n),seed:c.seed,level:1,..Self::default()};self.pattern();}
    fn round(&self)->&Round{&self.r}fn round_mut(&mut self)->&mut Round{&mut self.r}
    fn handle_input(&mut self,f:&[PlayerFrame]){self.input=f[self.actor];}
    fn update(&mut self,dt:f32){if !self.r.running(){return;}self.t+=dt;if self.done{self.gap+=dt;if self.gap>=0.85{
        self.actor+=1;if self.actor<self.r.score.len(){self.turn();}else if self.level<5{self.actor=0;self.level+=1;self.pattern();}else{let seats=(0..self.r.score.len()).collect::<Vec<_>>();
        if seats.len()==1&&self.r.score[0]==0{self.r.result=ResultState::Defeat;}else{self.r.timed_finish(&seats);}}}return;}
        if self.t<self.display_time(){return;}if let Some(d)=self.input.pressed[..4].iter().position(|v|*v){if d==self.seq[self.step]{self.step+=1;self.r.score[self.actor]+=1;self.r.cues.push(Fx::Tick);self.done=self.step==self.seq.len();}
        else{self.done=true;self.r.cues.push(Fx::Lose);}}
    }
    fn render(&self){bg(0x25223a);turn(self.actor,false);let watch=self.t<self.display_time();caption(&format!("{} / 5      {}",self.level,if self.done{"-"}else if watch{"WATCH"}else{"REPEAT"}),309.,74.,38.,WHITE);
        let beat=(self.t-0.65)/0.65;for(i,(dx,dy))in DIRECTIONS.iter().enumerate(){let(x,y)=(480.+*dx as f32*117.,280.+*dy as f32*100.);
            let lit=watch&&beat>=0.&&beat.fract()<0.72&&self.seq.get(beat as usize)==Some(&i);circle(x,y,44.,if lit{super::color(i)}else{Color::from_hex(0x4e495d)});
            let(d,e)=(*dx as f32,*dy as f32);draw_line(x-d*15.,y-e*15.,x+d*16.,y+e*16.,4.,WHITE);
            for sign in [-1.,1.]{draw_line(x+d*16.,y+e*16.,x+d*4.+e*9.*sign,y+e*4.-d*9.*sign,4.,WHITE);}
        }caption(&format!("{} / {}",self.step,self.seq.len()),449.,479.,25.,LIGHTGRAY);}
}
#[cfg(test)]mod tests{use super::*;#[test]fn wrong_direction_passes_to_next_seat(){let mut g=Echo::default();g.init(3,Config{seed:4});g.t=g.display_time();g.input.pressed[(g.seq[0]+1)%4]=true;g.update(0.01);g.update(1.);assert_eq!(g.actor,1);assert_eq!(g.r.score,[0,0,0]);}}
