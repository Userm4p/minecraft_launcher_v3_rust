use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::Serialize;

pub fn save_json_file<T: Serialize>(path: &Path, json_content: &T) -> Result<PathBuf> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(json_content)?;

    fs::write(path, json)?;

    Ok(path.to_path_buf())
}
