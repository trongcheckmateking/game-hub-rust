use macroquad::prelude::*;
pub const INK: Color = Color::new(0.07, 0.10, 0.10, 1.);
pub const PANEL: Color = Color::new(0.115, 0.15, 0.15, 1.);
pub const EDGE: Color = Color::new(0.24, 0.30, 0.29, 1.);
pub const PAPER: Color = Color::new(0.94, 0.95, 0.90, 1.);
pub const DIM: Color = Color::new(0.69, 0.75, 0.71, 1.);
pub const LIME: Color = Color::new(0.76, 0.94, 0.50, 1.);
pub fn camera(offset: Vec2) {
    let w = screen_width();
    let h = screen_height();
    let scale = (w / 1120.).min(h / 720.);
    let rw = (1120. * scale).round() as i32;
    let rh = (720. * scale).round() as i32;
    let mut c = Camera2D::from_display_rect(Rect::new(-offset.x, -offset.y, 1120., 720.));
    c.viewport = Some((
        ((w as i32 - rw) / 2),
        ((h as i32 - rh) / 2),
        rw.max(1),
        rh.max(1),
    ));
    set_camera(&c);
}
pub fn pointer() -> Vec2 {
    let (x, y) = mouse_position();
    let scale = (screen_width() / 1120.)
        .min(screen_height() / 720.)
        .max(0.001);
    vec2(
        (x - (screen_width() - 1120. * scale) / 2.) / scale,
        (y - (screen_height() - 720. * scale) / 2.) / scale,
    )
}
pub fn label(font: &Font, s: &str, x: f32, y: f32, size: u16, color: Color) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size,
            color,
            ..Default::default()
        },
    );
}
pub fn fitted(font: &Font, s: &str, r: Rect, size: u16, color: Color) {
    let w = measure_text(s, Some(font), size, 1.).width;
    let factor = if w > r.w - 18. { (r.w - 18.) / w } else { 1. };
    draw_text_ex(
        s,
        r.x + 9.,
        r.y + r.h / 2. + size as f32 * factor * 0.34,
        TextParams {
            font: Some(font),
            font_size: size,
            font_scale: factor,
            color,
            ..Default::default()
        },
    );
}
pub fn box_fill(r: Rect, c: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, c);
}
pub struct Nav {
    pub focus: usize,
    count: usize,
    total: usize,
    activate: bool,
}
impl Nav {
    pub fn new() -> Self {
        Self {
            focus: 0,
            count: 0,
            total: 0,
            activate: false,
        }
    }
    pub fn begin(&mut self, back: bool, next: bool, activate: bool) {
        if self.total > 0 {
            if next {
                self.focus = (self.focus + 1) % self.total;
            } else if back {
                self.focus = (self.focus + self.total - 1) % self.total;
            }
        }
        self.count = 0;
        self.activate = activate;
    }
    pub fn finish(&mut self) {
        self.total = self.count;
        self.focus = self.focus.min(self.total.saturating_sub(1));
    }
    pub fn is_next(&self) -> bool {
        self.focus == self.count
    }
    pub fn reset(&mut self) {
        self.focus = 0;
        self.total = 0;
    }
    pub fn hit(&mut self, r: Rect) -> bool {
        let idx = self.count;
        self.count += 1;
        let over = r.contains(pointer());
        if over && is_mouse_button_pressed(MouseButton::Left) {
            self.focus = idx;
            return true;
        }
        self.activate && self.focus == idx
    }
    pub fn button(&mut self, font: &Font, r: Rect, title: &str, active: bool) -> bool {
        let selected = self.focus == self.count;
        let over = r.contains(pointer());
        box_fill(
            r,
            if active {
                LIME
            } else if over {
                EDGE
            } else {
                PANEL
            },
        );
        draw_rectangle_lines(
            r.x,
            r.y,
            r.w,
            r.h,
            if selected { 3. } else { 1. },
            if selected { PAPER } else { EDGE },
        );
        fitted(font, title, r, 19, if active { INK } else { PAPER });
        self.hit(r)
    }
}
pub fn bi(en: bool, vi: &'static str, english: &'static str) -> &'static str {
    if en {
        english
    } else {
        vi
    }
}
