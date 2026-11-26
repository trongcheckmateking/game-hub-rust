use super::draw::*;
use crate::{core::save::Scores, games::{Era,CATALOG}};
use macroquad::prelude::{
    draw_rectangle_lines, draw_texture_ex, get_char_pressed, is_key_pressed, vec2,
    DrawTextureParams, Font, KeyCode, Rect, Texture2D, WHITE,
};
#[derive(Default)]
pub struct Lobby {
    party: usize,
    page: usize,
    style: usize,
    query: String,
    pub editing: bool,
}
pub enum LobbyAction {
    None,
    Settings,
    Choose(usize, usize),
    Quit,
}
impl Lobby {
    pub fn show(
        &mut self,
        font: &Font,
        nav: &mut Nav,
        art: &[Texture2D],
        scores: &Scores,
        en: bool,
    ) -> LobbyAction {
        let lang = |v, e| bi(en, v, e);
        let mut result = LobbyAction::None;
        label(
            font,
            lang("Chọn trò chơi", "Choose a game"),
            43.,
            146.,
            48,
            PAPER,
        );
        if nav.button(
            font,
            Rect::new(877., 99., 125., 43.),
            lang("Cài đặt", "Settings"),
            false,
        ) {
            result = LobbyAction::Settings;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if nav.button(
            font,
            Rect::new(1011., 99., 70., 43.),
            lang("Thoát", "Exit"),
            false,
        ) {
            result = LobbyAction::Quit;
        }
        for (i, n) in [0, 1, 2, 3, 4].iter().enumerate() {
            let s = if *n == 0 {
                lang("Tất cả", "All").to_owned()
            } else {
                format!("{n}P")
            };
            if nav.button(
                font,
                Rect::new(44. + i as f32 * 67., 176., 63., 40.),
                &s,
                self.party == *n,
            ) {
                self.party = *n;self.page=0;
            }
        }
        let style=if self.style==0 {lang("Mọi thời kỳ","All eras")}else{Era::ALL[self.style-1].label(en)};
        if nav.button(
            font,
            Rect::new(396., 176., 180., 40.),
            style,
            false,
        ) {
            self.style = (self.style + 1) % (Era::ALL.len()+1);self.page=0;
        }
        let sr = Rect::new(635., 176., 445., 40.);
        let search = if self.query.is_empty() {
            lang("Tìm trò chơi…", "Search games…").to_owned()
        } else {
            self.query.clone()
        };
        if nav.button(font, sr, &search, self.editing) {
            self.editing = !self.editing;
        }
        if self.editing {
            while let Some(c) = get_char_pressed() {
                if !c.is_control() && self.query.chars().count() < 32 {
                    self.query.push(c);
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                self.query.pop();
            }
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
                self.editing = false;
            }
        } else {
            while get_char_pressed().is_some() {}
        }
        let entries: Vec<_> = CATALOG
            .iter()
            .enumerate()
            .filter(|(_, g)| {
                (self.party == 0 || g.modes.contains(&self.party))
                    && (self.style == 0 || Era::ALL[self.style-1] == g.era)
                    && g.title.to_lowercase().contains(&self.query.to_lowercase())
            })
            .collect();
        if entries.is_empty() {
            label(
                font,
                lang(
                    "Chưa có trò phù hợp với bộ lọc.",
                    "No games match these filters.",
                ),
                45.,
                352.,
                28,
                PAPER,
            );
            if nav.button(
                font,
                Rect::new(45., 382., 190., 42.),
                lang("Xóa bộ lọc", "Clear filters"),
                false,
            ) {
                *self = Self::default();
            }
        }
        let pages = entries.len().div_ceil(3);
        self.page = self.page.min(pages.saturating_sub(1));
        if pages > 1 {
            if nav.button(font, Rect::new(817., 625., 50., 34.), "←", false) {
                self.page = (self.page + pages - 1) % pages;
            }
            label(
                font,
                &format!("{} / {}", self.page + 1, pages),
                882.,
                650.,
                18,
                DIM,
            );
            if nav.button(font, Rect::new(966., 625., 50., 34.), "→", false) {
                self.page = (self.page + 1) % pages;
            }
        }
        for (j, (idx, g)) in entries.into_iter().skip(self.page * 3).take(3).enumerate() {
            let mode = if g.modes.contains(&self.party) {
                self.party
            } else {
                g.modes[0]
            };
            let seats = g
                .modes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("/");
            let x = 44. + j as f32 * 352.;
            let card = Rect::new(x, 251., 333., 359.);
            box_fill(card, PANEL);
            draw_texture_ex(
                &art[idx],
                x,
                251.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(333., 176.)),
                    ..Default::default()
                },
            );
            let is_focus = nav.is_next();
            draw_rectangle_lines(
                card.x,
                card.y,
                card.w,
                card.h,
                if is_focus { 4. } else { 1. },
                if is_focus { LIME } else { EDGE },
            );
            label(
                font,
                &format!(
                    "{}P   /   {}",
                    seats,
                    g.era.label(en)
                ),
                x + 16.,
                458.,
                17,
                LIME,
            );
            label(font, g.title, x + 16., 501., 34, PAPER);
            let level = if g.difficulty == 1 {
                lang("Dễ", "Easy")
            } else {
                lang("Vừa", "Medium")
            };
            label(
                font,
                &format!("{}  ·  {}", lang("Độ khó", "Difficulty"), level),
                x + 16.,
                537.,
                18,
                DIM,
            );
            let best = scores
                .get(&format!("{}:{}", g.id, mode))
                .map(ToString::to_string)
                .unwrap_or_else(|| "—".into());
            label(
                font,
                &format!("{}  {}", lang("Kỷ lục", "Best"), best),
                x + 16.,
                582.,
                20,
                PAPER,
            );
            label(font, "→", x + 282., 584., 25, LIME);
            if nav.hit(card) {
                self.editing = false;
                result = LobbyAction::Choose(idx, mode);
            }
        }
        label(
            font,
            lang(
                "↑ ↓ chọn     Enter mở     M âm thanh     F11 toàn màn hình",
                "↑ ↓ browse     Enter open     M sound     F11 fullscreen",
            ),
            46.,
            668.,
            18,
            DIM,
        );
        result
    }
}
