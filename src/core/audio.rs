use super::{game::Fx, save::Settings};
use macroquad::audio::{
    load_sound_from_bytes, play_sound, set_sound_volume, stop_sound, PlaySoundParams, Sound,
};
use std::collections::HashMap;
macro_rules! clip {
    ($n:literal) => {
        (
            $n,
            &include_bytes!(concat!("../../assets/audio/", $n, ".wav"))[..],
        )
    };
}
pub struct Audio {
    sounds: HashMap<&'static str, Sound>,
    track: &'static str,
    volume: f32,
}
impl Audio {
    pub async fn load(tracks: impl IntoIterator<Item = (&'static str, &'static [u8])>) -> Self {
        let mut sounds = HashMap::new();
        for (k, b) in [
            clip!("hub"),
            clip!("click"),
            clip!("hit"),
            clip!("pickup"),
            clip!("explode"),
            clip!("tick"),
            clip!("win"),
            clip!("lose"),
        ]
        .into_iter()
        .chain(tracks)
        {
            if let Ok(s) = load_sound_from_bytes(b).await {
                sounds.insert(k, s);
            }
        }
        Self { sounds, track: "", volume: 0.0 }
    }
    pub fn music(&mut self, k: &'static str, s: &Settings) {
        let volume = if s.muted { 0.0 } else { s.bgm };
        if self.track != k {
            if let Some(old) = self.sounds.get(self.track) {
                stop_sound(old);
            }
            self.track = k;
            if let Some(track) = self.sounds.get(k) {
                play_sound(
                    track,
                    PlaySoundParams {
                        looped: true,
                        volume,
                    },
                );
            }
        } else if self.volume != volume {
            if let Some(track) = self.sounds.get(k) {
                set_sound_volume(track, volume);
            }
        }
        self.volume = volume;
    }
    pub fn effect(&self, k: &str, s: &Settings) {
        if !s.muted {
            if let Some(f) = self.sounds.get(k) {
                play_sound(
                    f,
                    PlaySoundParams {
                        looped: false,
                        volume: s.sfx,
                    },
                );
            }
        }
    }
    pub fn cue(&self, fx: Fx, s: &Settings) {
        self.effect(
            match fx {
                Fx::Hit => "hit",
                Fx::Pickup => "pickup",
                Fx::Explode => "explode",
                Fx::Lose => "lose",
                Fx::Tick => "tick",
            },
            s,
        );
    }
}
