use anyhow::{Error, Result};

use crate::app::models::{
    unified_version::UnifiedVersionManifest, version::VersionManifest,
    versions_below_26x::VersionManifestBelow26x,
};

pub async fn get_current_version_manifest(version_url: &str) -> Result<UnifiedVersionManifest> {
    print!("Fetching version manifest from {}...", version_url);
    let response = reqwest::get(version_url).await?.text().await?;

    if serde_json::from_str::<VersionManifest>(&response).is_ok() {
        let version_manifest: VersionManifest = serde_json::from_str(&response)?;
        return Ok(UnifiedVersionManifest::from_version_manifest(
            version_manifest,
        ));
    };

    if serde_json::from_str::<VersionManifestBelow26x>(&response).is_ok() {
        let version_manifest: VersionManifestBelow26x = serde_json::from_str(&response)?;
        return Ok(UnifiedVersionManifest::from_version_manifest_below_26x(
            version_manifest,
        ));
    };

    let value: serde_json::Value = serde_json::from_str(&response)?;

    if let Some(id) = value.get("id") {
        let message = format!("The version couldnt be parsed, current version: {:?}", id);
        return Err(Error::msg(message));
    }

    Err(Error::msg("The version couldnt be parsed"))
}
