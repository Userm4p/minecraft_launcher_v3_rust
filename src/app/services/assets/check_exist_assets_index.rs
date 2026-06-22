use sha1::{Digest, Sha1};
use std::fs;
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent, models::unified_version::UnifiedVersionManifest,
    utils::helpers::send_log,
};

pub fn check_exist_assets_index(
    manifest: &UnifiedVersionManifest,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
) -> bool {
    let assets_index_path = format!(
        "{}/assets/indexes/{}.json",
        installation_path, manifest.asset_index.id
    );

    if !fs::metadata(&assets_index_path).is_ok() {
        send_log(
            tx,
            format!(
                "Assets index not found for version {}. Expected at: {}",
                manifest.id, assets_index_path
            ),
        );
        return false;
    }

    let file_content = match fs::read(&assets_index_path) {
        Ok(content) => content,
        Err(err) => {
            send_log(
                tx,
                format!(
                    "Failed to read assets index for version {}: {}",
                    manifest.id, err
                ),
            );
            return false;
        }
    };

    let mut hasher = Sha1::new();
    hasher.update(&file_content);
    let result = hasher.finalize();
    let calculated_hash = hex::encode(result);
    if calculated_hash != manifest.asset_index.sha1 {
        send_log(
            tx,
            format!(
                "Assets index hash mismatch for version {}. Expected: {}, Calculated: {}",
                manifest.id, manifest.asset_index.sha1, calculated_hash
            ),
        );
        return false;
    }

    send_log(
        tx,
        format!(
            "Assets index exists and is valid for version {}.",
            manifest.id
        ),
    );
    true
}
