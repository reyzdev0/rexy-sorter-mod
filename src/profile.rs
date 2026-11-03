//! Profile per world: /Horizon/mods/rexy_sorter/profiles/<world>.json
use crate::config::{Config, SortMode, MOD_DIR};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub mode: SortMode,
    pub auto_refill: bool,
    pub locked_slots: Vec<usize>,
}

fn safe_name(world: &str) -> String {
    let s: String = world
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if s.is_empty() { "default".into() } else { s }
}

fn path(world: &str) -> PathBuf {
    PathBuf::from(MOD_DIR).join("profiles").join(format!("{}.json", safe_name(world)))
}

pub fn load(world: &str) -> Option<Profile> {
    serde_json::from_str(&fs::read_to_string(path(world)).ok()?).ok()
}

pub fn save(world: &str, p: &Profile) -> io::Result<()> {
    let file = path(world);
    if let Some(dir) = file.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(p).map_err(io::Error::other)?;
    fs::write(file, json)
}

impl Profile {
    pub fn from_config(c: &Config) -> Self {
        Profile { mode: c.mode, auto_refill: c.auto_refill, locked_slots: c.locked_slots.clone() }
    }
    pub fn apply(&self, c: &mut Config) {
        c.mode = self.mode;
        c.auto_refill = self.auto_refill;
        c.locked_slots = self.locked_slots.clone();
    }
}
