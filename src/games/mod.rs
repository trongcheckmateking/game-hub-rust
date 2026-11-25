pub mod pong;
pub mod snake;
pub mod fuse_yard;
use macroquad::prelude::Color;
pub const TITLES:[&str;3]=[pong::TITLE,snake::TITLE,fuse_yard::TITLE];
pub fn color(slot:usize)->Color{Color::from_hex([0xadf56e,0x79bdff,0xff927e,0xffdb74][slot])}
