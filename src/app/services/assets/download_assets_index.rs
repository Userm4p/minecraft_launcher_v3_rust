use std::fs;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    models::{assets::AssetsIndex, unified_version::UnifiedVersionManifest},
    services::assets::check_exist_assets_index::check_exist_assets_index,
};

pub async fn get_current_assets_index(
    manifest: &UnifiedVersionManifest,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
) -> Result<AssetsIndex> {
    if check_exist_assets_index(manifest, installation_path, tx) {
        let assets_index_path = format!(
            "{}/assets/indexes/{}.json",
            installation_path, manifest.asset_index.id
        );
        let file_content = fs::read(&assets_index_path)?;
        let assets_index: AssetsIndex = serde_json::from_slice(&file_content)?;
        return Ok(assets_index);
    }

    let assets_index_url = &manifest.asset_index.url;
    let response: AssetsIndex = reqwest::get(assets_index_url).await?.json().await?;

    return Ok(response);
}
