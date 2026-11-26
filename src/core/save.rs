use super::input::{can_bind, Bindings};
#[cfg(not(target_arch = "wasm32"))]
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
fn read<T: DeserializeOwned>(path: &Path) -> io::Result<T> {
    let mut bytes=Vec::new();
    use std::io::Read;
    fs::File::open(path)?.take(65537).read_to_end(&mut bytes)?;
    if bytes.len()>65536 { return Err(bad()); }
    serde_json::from_slice(&bytes).map_err(io::Error::from)
}
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
fn read_or_default<T: DeserializeOwned + Default>(file: &Path) -> io::Result<T> {
    match read(file) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        other => other,
    }
}
pub struct Store {
    pub root: PathBuf,
}
#[cfg(not(target_arch = "wasm32"))]
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
        if !s.valid() {
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
        if !s.valid() {
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
        if b.version != 1 || !b.save.valid() {
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

impl Save {
    fn valid(&self)->bool {
        self.settings.valid() && self.highscores.len()<=512 && self.highscores.keys().all(|key|{
            let Some((id,mode))=key.rsplit_once(':') else {return false};
            !id.is_empty() && id.len()<=48 && id.bytes().all(|c|c.is_ascii_lowercase()||c.is_ascii_digit()||c==b'-') && matches!(mode,"1"|"2"|"3"|"4")
        })
    }
}
#[cfg(target_arch = "wasm32")]
impl Store {
    pub fn local()->Self{Self{root:PathBuf::new()}}
    pub fn load(&self)->io::Result<Save>{
        let Some(bytes)=crate::platform::web::read(0)? else{return Ok(Save::default());};
        let save:Save=serde_json::from_slice(&bytes)?;
        if !save.valid(){return Err(bad());}Ok(save)
    }
    pub fn replace(&self,s:&Save)->io::Result<()>{if !s.valid(){return Err(bad());}crate::platform::web::write(0,&serde_json::to_vec(s)?)}
    pub fn settings(&self,s:&Settings)->io::Result<()>{let mut v=self.load()?;v.settings=s.clone();self.replace(&v)}
    pub fn scores(&self,s:&Scores)->io::Result<()>{let mut v=self.load()?;v.highscores=s.clone();self.replace(&v)}
    pub fn backup(&self)->PathBuf{PathBuf::from("GAME HUB / local backup")}
    pub fn export(&self,s:&Save)->io::Result<()>{crate::platform::web::write(1,&serde_json::to_vec(&Backup{version:1,save:s.clone()})?)}
    pub fn import(&self)->io::Result<Save>{let bytes=crate::platform::web::read(1)?.ok_or_else(bad)?;
        let b:Backup=serde_json::from_slice(&bytes)?;if b.version!=1{return Err(bad());}self.replace(&b.save)?;Ok(b.save)}
}
