use std::path::Path;

use tokio::sync::mpsc;

use crate::app::utils::helpers::{resolve_assets_path, send_log};

pub fn check_exist_assets(
    installation_path: &str,
    hash: &str,
    size: u64,
    tx: &mpsc::UnboundedSender<crate::app::app_state::LauncherEvent>,
) -> bool {
    let message = format!("Checking if asset exists: {}", hash);
    send_log(tx, &message);
    let asset_path = resolve_assets_path(installation_path, hash);

    if !Path::new(&asset_path).exists() {
        let message = format!("Asset {} does not exist", hash);
        send_log(tx, &message);
        return false;
    }

    let file_size = match std::fs::metadata(&asset_path) {
        Ok(metadata) => metadata.len(),
        Err(_) => {
            let message = format!("Failed to get metadata for asset {}", hash);
            send_log(tx, &message);
            return false;
        }
    };

    let message = format!("Asset {} exists: {}", hash, file_size == size);
    send_log(tx, &message);

    if file_size != size {
        let message = format!(
            "Asset {} size mismatch: expected {}, got {}",
            hash, size, file_size
        );
        send_log(tx, &message);
    }

    file_size == size
}
