use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::app::{
    infrastructure::filesystem::save_json_file,
    models::{assets::AssetsIndex, unified_version::UnifiedVersionManifest},
};

pub fn save_assets_index(
    minecraft_dir: &Path,
    assets_index: &AssetsIndex,
    manifest: &UnifiedVersionManifest,
) -> Result<PathBuf> {
    let path = minecraft_dir
        .join("assets")
        .join("indexes")
        .join(format!("{}.json", manifest.asset_index.id));

    save_json_file(&path, assets_index)
}
