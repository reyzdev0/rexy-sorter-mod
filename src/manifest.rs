//! Manifest .hnmod (build.sh menghasilkan file yang sama saat packaging)
use crate::config::MOD_DIR;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub entry: String,
}

impl Manifest {
    pub fn rexy() -> Self {
        Manifest {
            id: "rexy_sorter".into(),
            name: "Rexy Sorter".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            author: "Reyz".into(),
            entry: "librexy_sorter.so".into(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

/// Tulis manifest.json ke folder mod bila belum ada.
pub fn ensure_written() -> io::Result<()> {
    let p = Path::new(MOD_DIR).join("manifest.json");
    if p.exists() {
        return Ok(());
    }
    fs::create_dir_all(MOD_DIR)?;
    fs::write(p, Manifest::rexy().to_json())
}
