use std::path::Path;

use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    utils::helpers::{resolve_assets_path, send_log},
};

pub async fn download_asset(
    asset_hash: &str,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    send_log(tx, format!("Downloading asset: {}", asset_hash));

    let asset_url = format!(
        "https://resources.download.minecraft.net/{}/{}",
        &asset_hash[0..2],
        &asset_hash,
    );

    let response = client.get(&asset_url).send().await?;
    let bytes = response.bytes().await?;

    let asset_path_route = resolve_assets_path(installation_path, asset_hash);
    let asset_path = Path::new(&asset_path_route);

    if let Some(parent) = asset_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(asset_path, bytes)?;
    
    send_log(tx, format!("Downloaded asset: {}", asset_hash));

    Ok(())
}
