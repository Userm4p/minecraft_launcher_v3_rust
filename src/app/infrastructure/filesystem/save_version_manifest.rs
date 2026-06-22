use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::app::{
    infrastructure::filesystem::save_json_file, models::unified_version::UnifiedVersionManifest,
};

pub fn save_version_manifest(
    minecraft_dir: &Path,
    manifest: &UnifiedVersionManifest,
) -> Result<PathBuf> {
    let path = minecraft_dir
        .join("versions")
        .join(&manifest.id)
        .join(format!("{}.json", manifest.id));

    save_json_file(&path, manifest)
}
