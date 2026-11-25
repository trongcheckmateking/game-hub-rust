use super::input::{can_bind, Bindings};
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;
pub const SETTINGS_FILE: &str = "settings.json";
pub type Scores = BTreeMap<String, u32>;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct Settings {
    pub bgm: f32,
    pub sfx: f32,
    pub muted: bool,
    pub fullscreen: bool,
    pub english: bool,
    pub retro: bool,
    pub keys: Bindings,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            bgm: 0.32,
            sfx: 0.65,
            muted: false,
            fullscreen: false,
            english: false,
            retro: false,
            keys: Bindings::new(),
        }
    }
}
impl Settings {
    pub fn valid(&self) -> bool {
        (0.0..=1.0).contains(&self.bgm)
            && (0.0..=1.0).contains(&self.sfx)
            && self.keys.iter().all(|(k, v)| {
                let Some((p, a)) = k.split_once('.') else {
                    return false;
                };
                match (p.parse::<usize>(), a.parse::<usize>()) {
                    (Ok(p), Ok(a)) => format!("{p}.{a}") == *k && can_bind(p, a, v, &self.keys),
                    _ => false,
                }
            })
    }
}
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Save {
    pub settings: Settings,
    pub highscores: Scores,
}
#[derive(Serialize, Deserialize)]
struct Backup {
    version: u8,
    save: Save,
}
fn bad() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Invalid GAME HUB data")
}
fn read<T: DeserializeOwned>(path: &Path) -> io::Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(io::Error::from)
}
fn atomic<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let root = path.parent().ok_or_else(bad)?;
    fs::create_dir_all(root)?;
    let mut tmp = NamedTempFile::new_in(root)?;
    serde_json::to_writer_pretty(&mut tmp, value)?;
    tmp.flush()?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| e.error)?;
    Ok(())
}
fn read_or_default<T: DeserializeOwned + Default>(file: &Path) -> io::Result<T> {
    match read(file) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        other => other,
    }
}
pub struct Store {
    pub root: PathBuf,
}
impl Store {
    pub fn local() -> Self {
        Self {
            root: ProjectDirs::from("dev", "Trong", "GameHub")
                .map(|p| p.data_local_dir().to_owned())
                .unwrap_or_else(|| PathBuf::from("gamehub-data")),
        }
    }
    pub fn load(&self) -> io::Result<Save> {
        // A pending transaction is replayable if import/reset was interrupted.
        let journal = self.root.join("pending.json");
        if journal.exists() {
            let s: Save = read(&journal)?;
            self.replace(&s)?;
            return Ok(s);
        }
        let s = Save {
            settings: read_or_default(&self.root.join(SETTINGS_FILE))?,
            highscores: read_or_default(&self.root.join("highscores.json"))?,
        };
        if !s.settings.valid() {
            return Err(bad());
        }
        Ok(s)
    }
    pub fn settings(&self, v: &Settings) -> io::Result<()> {
        if !v.valid() {
            return Err(bad());
        }
        atomic(&self.root.join(SETTINGS_FILE), v)
    }
    pub fn scores(&self, v: &Scores) -> io::Result<()> {
        atomic(&self.root.join("highscores.json"), v)
    }
    pub fn replace(&self, s: &Save) -> io::Result<()> {
        if !s.settings.valid() {
            return Err(bad());
        }
        let j = self.root.join("pending.json");
        atomic(&j, s)?;
        self.settings(&s.settings)?;
        self.scores(&s.highscores)?;
        fs::remove_file(j)
    }
    pub fn backup(&self) -> PathBuf {
        self.root.join("gamehub-backup.json")
    }
    pub fn export(&self, s: &Save) -> io::Result<()> {
        atomic(
            &self.backup(),
            &Backup {
                version: 1,
                save: s.clone(),
            },
        )
    }
    pub fn import(&self) -> io::Result<Save> {
        let b: Backup = read(&self.backup())?;
        if b.version != 1 || !b.save.settings.valid() {
            return Err(bad());
        }
        self.replace(&b.save)?;
        Ok(b.save)
    }
}
pub fn record(scores: &mut Scores, id: &str, n: usize, points: &[u32]) {
    let best = scores.entry(format!("{id}:{n}")).or_default();
    *best = (*best).max(points.iter().copied().max().unwrap_or(0));
}
