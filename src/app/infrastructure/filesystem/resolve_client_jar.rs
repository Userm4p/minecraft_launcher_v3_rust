use anyhow::Result;
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent, infrastructure::filesystem::get_version_sha1,
    models::unified_version::UnifiedVersionManifest, services::download_client_jar,
    utils::helpers::send_log,
};

pub async fn resolve_client_jar(
    manifest: &UnifiedVersionManifest,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
) -> Result<()> {
    send_log(tx, "Checking client JAR...");

    match get_version_sha1(installation_path, &manifest.id) {
        Some(sha1) if sha1 == manifest.downloads.client.sha1 => {
            send_log(tx, "Client JAR already exists and is valid");
        }
        Some(_) => {
            send_log(tx, "Client JAR is corrupted, redownloading...");

            download_client_jar(manifest, installation_path).await?;

            send_log(tx, "Client JAR downloaded successfully");
        }
        None => {
            send_log(tx, "Client JAR not found, downloading...");

            download_client_jar(manifest, installation_path).await?;

            send_log(tx, "Client JAR downloaded successfully");
        }
    }

    Ok(())
}
