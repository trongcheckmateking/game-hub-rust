pub mod pong;
pub mod snake;
pub mod fuse_yard;
use macroquad::prelude::Color;
pub fn color(slot:usize)->Color{Color::from_hex([0xadf56e,0x79bdff,0xff927e,0xffdb74][slot])}
use crate::core::game::{Config,MiniGame};
pub struct Entry{
    pub id:&'static str,pub title:&'static str,pub modes:&'static [usize],pub retro:bool,pub difficulty:u8,
    pub art:&'static [u8],pub music:&'static [u8],pub rules:[&'static [&'static str];2],factory:fn()->Box<dyn MiniGame>,
}
impl Entry{
    pub fn start(&self,players:usize,seed:u64)->Box<dyn MiniGame>{assert!(self.modes.contains(&players));let mut g=(self.factory)();g.init(players,Config{seed});g}
}
pub const CATALOG:&[Entry]=&[
    Entry{id:"pong",title:pong::TITLE,modes:&[2],retro:true,difficulty:1,art:include_bytes!("../../assets/art/pong.png"),music:include_bytes!("../../assets/audio/pong.wav"),
        rules:[&["Lên / xuống để di chuyển vợt.","Đánh bằng mép vợt để đổi góc bóng.","Đưa bóng qua vạch cuối sân. Ai đạt 7 điểm trước sẽ thắng."],
        &["Move your paddle up or down.","Strike near the tip to change the angle.","Pass the far baseline to score. First to 7 wins."]],factory:||Box::new(pong::Pong::default())},
    Entry{id:"snake",title:snake::TITLE,modes:&[2],retro:true,difficulty:2,art:include_bytes!("../../assets/art/snake.png"),music:include_bytes!("../../assets/audio/snake.wav"),
        rules:[&["Rẽ bằng bốn hướng. Rắn luôn tiến và không thể quay đầu ngay.","Ăn mồi +2. Đụng tường hoặc thân rắn sẽ bị loại.","Sống sót +5 và thắng. Cùng bị loại thì hòa. Hết 90 giây xét điểm."],
        &["Steer in four directions. No instant reversal.","Fruit gives 2 points. Walls or bodies eliminate you.","Sole survivor wins (+5). Double crash draws. At 90s highest score wins."]],factory:||Box::new(snake::Snake::default())},
    Entry{id:"fuse-yard",title:fuse_yard::TITLE,modes:&[4],retro:false,difficulty:2,art:include_bytes!("../../assets/art/fuse-yard.png"),music:include_bytes!("../../assets/audio/fuse-yard.wav"),
        rules:[&["A đặt lõi. B đổi trục ngang / dọc cho lõi tiếp theo.","Sau 1,6 giây, xung đi 3 ô mỗi phía, phá kiện (+1) và kích lõi khác.","Người cuối cùng thắng (+5). Hết 100 giây xét điểm người còn sống."],
        &["A drops a core. B switches its horizontal / vertical axis.","After 1.6s the pulse travels 3 tiles, breaks crates (+1) and chains cores.","Last survivor wins (+5). At 100s compare surviving scores."]],factory:||Box::new(fuse_yard::FuseYard::default())}
];
