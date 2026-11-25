use super::draw::*;
use crate::core::{
    input::{can_bind, names, ACTIONS},
    save::{Save, Store},
};
use macroquad::prelude::{
    draw_rectangle, get_last_key_pressed, is_mouse_button_down, set_fullscreen, Font, KeyCode,
    MouseButton, Rect,
};
#[derive(Default)]
pub struct Preferences {
    player: usize,
    listen: Option<usize>,
    confirm: bool,
    pub note: String,
}
impl Preferences {
    pub fn capturing(&self) -> bool {
        self.listen.is_some()
    }
    pub fn show(&mut self, font: &Font, nav: &mut Nav, save: &mut Save, store: &Store) -> bool {
        let en = save.settings.english;
        let lang = |v, e| bi(en, v, e);
        label(font, lang("Cài đặt", "Settings"), 44., 128., 40, PAPER);
        if let Some(a) = self.listen {
            label(
                font,
                lang(
                    "Nhấn phím mới hoặc Esc để hủy",
                    "Press a key or Esc to cancel",
                ),
                48.,
                222.,
                30,
                LIME,
            );
            if let Some(k) = get_last_key_pressed() {
                let name = format!("{k:?}");
                if k == KeyCode::Escape {
                    self.listen = None;
                } else if can_bind(self.player, a, &name, &save.settings.keys) {
                    save.settings
                        .keys
                        .insert(format!("{}.{}", self.player, a), name);
                    self.listen = None;
                    self.note.clear();
                    if let Err(e) = store.settings(&save.settings) {
                        self.note = e.to_string();
                    }
                } else {
                    self.note = lang(
                        "Phím trùng, dành riêng hoặc chưa hỗ trợ.",
                        "Key is assigned, reserved or unsupported.",
                    )
                    .into();
                }
            }
            label(font, &self.note, 48., 284., 20, DIM);
            return false;
        }
        let before = save.settings.clone();
        let mut back = nav.button(
            font,
            Rect::new(925., 90., 145., 42.),
            lang("Quay lại", "Back"),
            false,
        );
        for (i, name) in ["BGM", "SFX"].iter().enumerate() {
            let y = 163. + i as f32 * 57.;
            label(font, name, 48., y + 26., 21, PAPER);
            let v = if i == 0 {
                &mut save.settings.bgm
            } else {
                &mut save.settings.sfx
            };
            let rail = Rect::new(180., y, 220., 38.);
            if is_mouse_button_down(MouseButton::Left) && rail.contains(pointer()) {
                *v = ((pointer().x - rail.x) / rail.w).clamp(0., 1.);
            }
            draw_rectangle(rail.x, rail.y + 15., rail.w, 8., EDGE);
            draw_rectangle(rail.x, rail.y + 15., rail.w * *v, 8., LIME);
            if nav.button(font, Rect::new(131., y, 34., 38.), "−", false) {
                *v = (*v - 0.05).max(0.);
            }
            if nav.button(font, Rect::new(417., y, 34., 38.), "+", false) {
                *v = (*v + 0.05).min(1.);
            }
            label(
                font,
                &format!("{}%", (*v * 100.).round()),
                463.,
                y + 26.,
                18,
                DIM,
            );
        }
        if nav.button(
            font,
            Rect::new(48., 286., 225., 43.),
            lang("Tắt tiếng", "Mute"),
            save.settings.muted,
        ) {
            save.settings.muted = !save.settings.muted;
        }
        if nav.button(
            font,
            Rect::new(288., 286., 225., 43.),
            lang("Toàn màn hình", "Fullscreen"),
            save.settings.fullscreen,
        ) {
            save.settings.fullscreen = !save.settings.fullscreen;
            set_fullscreen(save.settings.fullscreen);
        }
        if nav.button(
            font,
            Rect::new(48., 343., 225., 43.),
            if en {
                "English / VI"
            } else {
                "Tiếng Việt / EN"
            },
            false,
        ) {
            save.settings.english = !en;
        }
        if nav.button(
            font,
            Rect::new(288., 343., 225., 43.),
            if save.settings.retro {
                "Retro"
            } else {
                "Modern"
            },
            false,
        ) {
            save.settings.retro = !save.settings.retro;
        }
        label(
            font,
            lang("Phím điều khiển", "Key bindings"),
            571.,
            188.,
            26,
            PAPER,
        );
        for p in 0..4 {
            if nav.button(
                font,
                Rect::new(572. + p as f32 * 88., 208., 78., 36.),
                &format!("P{}", p + 1),
                self.player == p,
            ) {
                self.player = p;
            }
        }
        let lab = if en {
            ["Up", "Down", "Left", "Right", "Action A", "Action B"]
        } else {
            ["Lên", "Xuống", "Trái", "Phải", "Hành động A", "Hành động B"]
        };
        for (a, act) in ACTIONS.iter().enumerate() {
            let y = 257. + a as f32 * 40.;
            label(font, lab[a], 573., y + 25., 19, DIM);
            let value = names(self.player, a, 4, &save.settings.keys).join(" / ");
            if nav.button(font, Rect::new(755., y, 305., 34.), &value, false) {
                self.listen = Some(a);
                self.note = format!("P{} · {}", self.player + 1, act);
            }
        }
        label(
            font,
            lang("Dữ liệu trên máy", "Local save data"),
            48.,
            454.,
            26,
            PAPER,
        );
        let path = store.backup().display().to_string();
        fitted(font, &path, Rect::new(40., 475., 520., 35.), 15, DIM);
        if nav.button(
            font,
            Rect::new(48., 525., 153., 40.),
            lang("Xuất backup", "Export backup"),
            false,
        ) {
            self.note = store
                .export(save)
                .map(|_| lang("Đã xuất backup", "Backup written").to_owned())
                .unwrap_or_else(|e| e.to_string());
        }
        if nav.button(
            font,
            Rect::new(211., 525., 153., 40.),
            lang("Nhập backup", "Import backup"),
            false,
        ) {
            match store.import() {
                Ok(s) => {
                    *save = s;
                    set_fullscreen(save.settings.fullscreen);
                    self.note = lang("Đã nhập backup", "Backup restored").into();
                }
                Err(e) => self.note = e.to_string(),
            }
        }
        if nav.button(
            font,
            Rect::new(375., 525., 160., 40.),
            lang("Xóa dữ liệu", "Reset save"),
            false,
        ) {
            self.confirm = !self.confirm;
        }
        if self.confirm {
            if nav.button(
                font,
                Rect::new(48., 577., 210., 40.),
                lang("Xác nhận xóa", "Confirm reset"),
                false,
            ) {
                let s = Save::default();
                match store.replace(&s) {
                    Ok(()) => {
                        *save = s;
                        set_fullscreen(false);
                        self.note = lang("Đã đặt lại dữ liệu", "Save reset").into();
                    }
                    Err(e) => self.note = e.to_string(),
                };
                self.confirm = false;
            }
            if nav.button(
                font,
                Rect::new(270., 577., 130., 40.),
                lang("Hủy", "Cancel"),
                false,
            ) {
                self.confirm = false;
            }
        }
        if before != save.settings {
            if let Err(e) = store.settings(&save.settings) {
                self.note = e.to_string();
            }
        }
        if !self.note.is_empty() {
            fitted(font, &self.note, Rect::new(39., 645., 1030., 35.), 18, LIME);
        }
        back |= macroquad::prelude::is_key_pressed(KeyCode::Escape);
        back
    }
}
