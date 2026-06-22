use anyhow::Result;

use crate::app::models::{
    unified_version::UnifiedVersionManifest, version::VersionManifest,
    versions_below_26x::VersionManifestBelow26x,
};

pub async fn get_current_version_manifest(version_url: &str) -> Result<UnifiedVersionManifest> {
    print!("Fetching version manifest from {}...", version_url);
    let response = reqwest::get(version_url).await?.text().await?;

    if serde_json::from_str::<VersionManifest>(&response).is_ok() {
        let version_manifest: VersionManifest = serde_json::from_str(&response)?;
        Ok(UnifiedVersionManifest::from_version_manifest(
            version_manifest,
        ))
    } else {
        let version_manifest_below_26x: VersionManifestBelow26x = serde_json::from_str(&response)?;
        Ok(UnifiedVersionManifest::from_version_manifest_below_26x(
            version_manifest_below_26x,
        ))
    }
}
