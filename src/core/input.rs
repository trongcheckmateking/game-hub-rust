//! Translate devices to six actions per slot. Keep button edges until a simulation tick.
#[cfg(not(target_arch = "wasm32"))]
use gilrs::{Axis, Button, GamepadId, Gilrs};
use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};
use std::collections::BTreeMap;
pub const ACTIONS: [&str; 6] = ["up", "down", "left", "right", "a", "b"];
pub type Bindings = BTreeMap<String, String>;
#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerFrame {
    pub held: [bool; 6],
    pub pressed: [bool; 6],
}
pub fn names(slot: usize, action: usize, n: usize, keys: &Bindings) -> Vec<String> {
    if let Some(key) = keys.get(&format!("{slot}.{action}")) {
        return vec![key.clone()];
    }
    let base = [
        ["W", "S", "A", "D", "F", "G"],
        ["Up", "Down", "Left", "Right", "Enter", "RightShift"],
        ["I", "K", "J", "L", "U", "O"],
        ["Kp8", "Kp5", "Kp4", "Kp6", "Kp7", "Kp9"],
    ];
    let mut v = vec![base[slot][action].to_owned()];
    if slot == 0 && action == 4 {
        v.push("Space".into());
    }
    if slot == 1 && action == 4 {
        v.push("Kp0".into());
    }
    if slot == 1 && n == 2 && action >= 4 {
        v.push(["K", "L"][action - 4].into());
    }
    v
}
pub fn allowed_key(code: &str) -> bool {
    (code.len() == 1 && code.as_bytes()[0].is_ascii_uppercase() && !["R", "M"].contains(&code))
        || [
            "Space",
            "Enter",
            "Up",
            "Down",
            "Left",
            "Right",
            "LeftShift",
            "RightShift",
            "Comma",
            "Period",
            "Slash",
            "Semicolon",
            "Apostrophe",
            "Minus",
            "Equal",
            "LeftBracket",
            "RightBracket",
            "KpEnter",
            "KpDecimal",
            "KpAdd",
            "KpSubtract",
        ]
        .contains(&code)
        || ["Kp", "Key"].iter().any(|p| {
            code.strip_prefix(*p)
                .is_some_and(|s| s.len() == 1 && s.as_bytes()[0].is_ascii_digit())
        })
}
pub fn can_bind(slot: usize, action: usize, code: &str, keys: &Bindings) -> bool {
    (slot < 4 && action < 6 && allowed_key(code))
        && (1..=4).all(|n| {
            slot >= n
                || (0..n).all(|p| {
                    (0..6).all(|a| {
                        (slot == p && action == a)
                            || !names(p, a, n, keys).iter().any(|k| k == code)
                    })
                })
        })
}
// Resolve saved string bindings once per configuration, rather than formatting keys each frame.
fn key_code(name: &str) -> Option<KeyCode> {
    macro_rules! keys {
        ($($key:ident),+) => { match name {
            $(stringify!($key) => Some(KeyCode::$key),)+
            _ => None,
        } };
    }
    keys!(
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Key0,
        Key1,
        Key2,
        Key3,
        Key4,
        Key5,
        Key6,
        Key7,
        Key8,
        Key9,
        Kp0,
        Kp1,
        Kp2,
        Kp3,
        Kp4,
        Kp5,
        Kp6,
        Kp7,
        Kp8,
        Kp9,
        KpEnter,
        KpDecimal,
        KpAdd,
        KpSubtract,
        Up,
        Down,
        Left,
        Right,
        Enter,
        Space,
        LeftShift,
        RightShift,
        Comma,
        Period,
        Slash,
        Semicolon,
        Apostrophe,
        Minus,
        Equal,
        LeftBracket,
        RightBracket
    )
}

#[derive(Default)]
struct Keyboard {
    party: usize,
    bindings: Bindings,
    codes: [[Vec<KeyCode>; 6]; 4],
}
impl Keyboard {
    fn configure(&mut self, party: usize, bindings: &Bindings) {
        if self.party == party && self.bindings == *bindings {
            return;
        }
        self.party = party;
        self.bindings.clone_from(bindings);
        self.codes = std::array::from_fn(|p| {
            std::array::from_fn(|a| {
                names(p, a, party, bindings)
                    .iter()
                    .filter_map(|name| key_code(name))
                    .collect()
            })
        });
    }
}
pub struct Input {
    keyboard: Keyboard,
    #[cfg(not(target_arch = "wasm32"))]
    pads: Option<Gilrs>,
    #[cfg(not(target_arch = "wasm32"))]
    slots: [Option<GamepadId>; 4],
    previous: [[bool; 6]; 4],
    pending: [PlayerFrame; 4],
    pub menu: [bool; 6],
    pub start: bool,
    prev_start: bool,
}
impl Default for Input {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            pads: Gilrs::new().ok(),
            keyboard: Keyboard::default(),
            #[cfg(not(target_arch = "wasm32"))]
            slots: [None; 4],
            previous: [[false; 6]; 4],
            pending: [PlayerFrame::default(); 4],
            menu: [false; 6],
            start: false,
            prev_start: false,
        }
    }
}
impl Input {
    pub fn count(&self) -> usize {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.slots.iter().flatten().count()
        }
        #[cfg(target_arch = "wasm32")]
        {
            crate::platform::web::pads()
        }
    }
    pub fn poll(&mut self, n: usize, keys: &Bindings) {
        self.keyboard.configure(n, keys);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(g) = &mut self.pads {
            while g.next_event().is_some() {}
            for slot in &mut self.slots {
                if slot.is_some_and(|id| !g.gamepad(id).is_connected()) {
                    *slot = None;
                }
            }
            for (id, _) in g.gamepads() {
                if !self.slots.contains(&Some(id)) {
                    if let Some(s) = self.slots.iter_mut().find(|s| s.is_none()) {
                        *s = Some(id);
                    }
                }
            }
        }
        self.menu = [false; 6];
        let mut start = false;
        for p in 0..4 {
            let mut joy = [false; 6];
            #[cfg(not(target_arch = "wasm32"))]
            if let (Some(g), Some(id)) = (&self.pads, self.slots[p]) {
                let pad = g.gamepad(id);
                joy = [
                    pad.is_pressed(Button::DPadUp) || pad.value(Axis::LeftStickY) > 0.4,
                    pad.is_pressed(Button::DPadDown) || pad.value(Axis::LeftStickY) < -0.4,
                    pad.is_pressed(Button::DPadLeft) || pad.value(Axis::LeftStickX) < -0.4,
                    pad.is_pressed(Button::DPadRight) || pad.value(Axis::LeftStickX) > 0.4,
                    pad.is_pressed(Button::South),
                    pad.is_pressed(Button::East),
                ];
                start |= pad.is_pressed(Button::Start);
            }
            #[cfg(target_arch = "wasm32")]
            let (bits, taps) = crate::platform::web::input(p);
            #[cfg(target_arch = "wasm32")]
            {
                joy = std::array::from_fn(|a| bits & (1 << a) != 0);
                start |= (bits | taps) & 64 != 0;
            }
            for (a, &j) in joy.iter().enumerate() {
                let v = &self.keyboard.codes[p][a];
                let edge = j && !self.previous[p][a];
                #[cfg(target_arch = "wasm32")]
                let edge = edge || taps & (1 << a) != 0;
                self.menu[a] |= edge;
                self.previous[p][a] = j;
                self.pending[p].held[a] = j || v.iter().copied().any(is_key_down);
                self.pending[p].pressed[a] |= edge || v.iter().copied().any(is_key_pressed);
            }
        }
        self.start = start && !self.prev_start;
        self.prev_start = start;
    }
    pub fn take(&mut self) -> [PlayerFrame; 4] {
        let f = self.pending;
        self.clear();
        f
    }
    pub fn clear(&mut self) {
        for f in &mut self.pending {
            f.pressed = [false; 6];
        }
    }
}
pub fn pressed(k: KeyCode) -> bool {
    macroquad::prelude::is_key_pressed(k)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyboard_refreshes_when_seats_or_bindings_change() {
        let mut layout = Keyboard::default();
        let mut bindings = Bindings::new();
        layout.configure(2, &bindings);
        assert!(layout.codes[1][4].contains(&KeyCode::K));
        layout.configure(3, &bindings);
        assert!(!layout.codes[1][4].contains(&KeyCode::K));
        bindings.insert("2.0".into(), "Z".into());
        layout.configure(3, &bindings);
        assert_eq!(layout.codes[2][0], [KeyCode::Z]);
    }
}
