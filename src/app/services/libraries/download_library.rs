use std::path::{Path, PathBuf};

use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    models::unified_version::{UnifiedLibrary, UnifiedVersionManifest},
    services::libraries::extract_native_library::extract_native_library,
    utils::helpers::{is_native_library, send_log},
};

pub async fn download_library(
    library: &UnifiedLibrary,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
    client: &reqwest::Client,
    manifest: &UnifiedVersionManifest,
) -> anyhow::Result<()> {
    let library_jar_url = library.downloads.artifact.url.clone();

    let response = client.get(&library_jar_url).send().await?;
    let bytes = response.bytes().await?;

    let jar_path: PathBuf;

    if let Some(library_download_path) = &library.downloads.artifact.path {
        jar_path = Path::new(installation_path)
            .join("libraries")
            .join(library_download_path);

        if let Some(parent) = jar_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&jar_path, bytes)?;
    } else {
        send_log(
            tx,
            format!(
                "Failed to download library: {}. No download path specified.",
                library.name
            ),
        );

        return Ok(());
    }

    if is_native_library(library) {
        send_log(tx, format!("Extracting natives: {}", library.name));

        let natives_dir = Path::new(installation_path)
            .join("versions")
            .join(&manifest.id)
            .join("natives");

        std::fs::create_dir_all(&natives_dir)?;

        extract_native_library(&jar_path, &natives_dir)?;

        send_log(tx, format!("Natives extracted: {}", library.name));
    }

    Ok(())
}
