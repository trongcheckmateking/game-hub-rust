use super::draw::*;
use crate::{
    core::save::Scores,
    games::{Era, CATALOG},
};
use macroquad::prelude::{
    draw_texture_ex, get_char_pressed, get_frame_time, is_key_pressed, vec2, DrawTextureParams,
    Font, KeyCode, Rect, Texture2D, WHITE,
};
#[derive(Default)]
pub struct Lobby {
    party: usize,
    page: usize,
    style: usize,
    query: String,
    pub editing: bool,
    filter: Option<(usize, usize, String)>,
    matches: Vec<usize>,
    seats: Vec<String>,
    lift: [f32; 3],
    visible: [Option<usize>; 3],
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
            let s = [lang("Tất cả", "All"), "1P", "2P", "3P", "4P"][i];
            if nav.button(
                font,
                Rect::new(44. + i as f32 * 67., 176., 63., 40.),
                s,
                self.party == *n,
            ) {
                self.party = *n;
                self.page = 0;
            }
        }
        let style = if self.style == 0 {
            lang("Mọi thời kỳ", "All eras")
        } else {
            Era::ALL[self.style - 1].label(en)
        };
        if nav.button(font, Rect::new(396., 176., 180., 40.), style, false) {
            self.style = (self.style + 1) % (Era::ALL.len() + 1);
            self.page = 0;
        }
        let sr = Rect::new(635., 176., 445., 40.);
        let search = if self.query.is_empty() {
            lang("Tìm trò chơi…", "Search games…")
        } else {
            self.query.as_str()
        };
        if nav.button(font, sr, search, self.editing) {
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
        if self.seats.is_empty() {
            self.seats = CATALOG
                .iter()
                .map(|g| {
                    let first = g.modes[0];
                    let last = g.modes[g.modes.len() - 1];
                    if g.modes.len() == 1 {
                        format!("{first}P")
                    } else if g.modes.windows(2).all(|n| n[1] == n[0] + 1) {
                        format!("{first}–{last}P")
                    } else {
                        format!(
                            "{}P",
                            g.modes
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join("/")
                        )
                    }
                })
                .collect();
        }
        if !self.filter.as_ref().is_some_and(|(party, style, q)| {
            *party == self.party && *style == self.style && q == &self.query
        }) {
            let query = self.query.to_lowercase();
            self.matches.clear();
            self.matches
                .extend(CATALOG.iter().enumerate().filter_map(|(i, g)| {
                    ((self.party == 0 || g.modes.contains(&self.party))
                        && (self.style == 0 || Era::ALL[self.style - 1] == g.era)
                        && (query.is_empty() || g.title.to_lowercase().contains(&query)))
                    .then_some(i)
                }));
            self.filter = Some((self.party, self.style, self.query.clone()));
        }
        if self.matches.is_empty() {
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
        let pages = self.matches.len().div_ceil(3);
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
        for (j, &idx) in self.matches.iter().skip(self.page * 3).take(3).enumerate() {
            let g = &CATALOG[idx];
            let mode = if g.modes.contains(&self.party) {
                self.party
            } else {
                g.modes[0]
            };
            let seats = &self.seats[idx];
            let x = 44. + j as f32 * 352.;
            let hitbox = Rect::new(x, 251., 333., 359.);
            let focus = nav.is_next();
            let active = focus || hitbox.contains(pointer());
            if self.visible[j] != Some(idx) {
                self.visible[j] = Some(idx);
                self.lift[j] = 0.;
            }
            let target = if active { 1. } else { 0. };
            self.lift[j] +=
                (target - self.lift[j]) * (1. - (-get_frame_time().min(0.1) * 16.).exp());
            let dy = -5. * self.lift[j];
            let card = Rect::new(x, 251. + dy, 333., 359.);
            nav.panel(
                card,
                20.,
                if focus {
                    LIME
                } else if active {
                    DIM
                } else {
                    EDGE
                },
                PANEL,
                if focus { 2.5 } else { 1. },
            );
            draw_texture_ex(
                &art[idx],
                x + 8.,
                card.y + 8.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(317., 167.)),
                    ..Default::default()
                },
            );
            label(
                font,
                &format!("{}   /   {}", seats, g.era.label(en)),
                x + 16.,
                458. + dy,
                17,
                LIME,
            );
            label(font, g.title, x + 16., 501. + dy, 34, PAPER);
            let level = if g.difficulty == 1 {
                lang("Dễ", "Easy")
            } else {
                lang("Vừa", "Medium")
            };
            label(
                font,
                &format!("{}  ·  {}", lang("Độ khó", "Difficulty"), level),
                x + 16.,
                537. + dy,
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
                582. + dy,
                20,
                PAPER,
            );
            label(font, "→", x + 282., 584. + dy, 25, LIME);
            if nav.hit(hitbox) {
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
