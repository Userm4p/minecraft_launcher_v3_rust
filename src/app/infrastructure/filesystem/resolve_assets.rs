use anyhow::Result;
use futures::{StreamExt, stream};
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    models::assets::AssetsIndex,
    services::assets::{check_exist_assets::check_exist_assets, download_asset::download_asset},
    utils::helpers::send_log,
};

pub async fn resolve_assets(
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
    assets_index: AssetsIndex,
) -> Result<()> {
    let client = reqwest::Client::new();
    let assets: Vec<_> = assets_index
        .objects
        .into_iter()
        .filter(|(_, asset)| {
            !check_exist_assets(installation_path, &asset.hash, asset.size as u64, &tx)
        })
        .collect();

    let total = assets.len();

    send_log(&tx, format!("Downloading {} assets...", total));

    stream::iter(assets.into_iter().enumerate())
        .map(|(index, (_name, asset))| {
            let tx = tx.clone();
            let client = client.clone();

            async move {
                if index % 100 == 0 {
                    send_log(&tx, format!("Assets progress: {}/{}", index, total));
                }

                download_asset(&asset.hash, installation_path, &tx, &client).await
            }
        })
        .buffer_unordered(64)
        .collect::<Vec<_>>()
        .await;

    send_log(&tx, "Assets resolved");

    Ok(())
}
