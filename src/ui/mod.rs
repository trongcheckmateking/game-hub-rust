use macroquad::prelude::*;
pub async fn run() {
    loop {
        clear_background(Color::from_hex(0x182326));
        draw_text("GAME HUB", 48.0, 92.0, 58.0, WHITE);
        for (i, game) in crate::games::TITLES.iter().enumerate() {
            draw_text(game, 50.0, 160.0 + 52.0 * i as f32, 32.0, LIME);
        }
        next_frame().await;
    }
}
