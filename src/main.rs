#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
use macroquad::prelude::*;
fn window() -> Conf {
    Conf { window_title: "GAME HUB".into(), window_width: 1120, window_height: 700, ..Default::default() }
}
#[macroquad::main(window)]
async fn main() { game_hub::ui::run().await; }
