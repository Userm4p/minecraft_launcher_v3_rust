use anyhow::Result;
use futures::{StreamExt, stream};
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    models::unified_version::UnifiedVersionManifest,
    services::libraries::{
        check_exist_library::check_exist_library, download_library::download_library,
        get_necessary_libraries::get_necessary_libraries,
    },
    utils::helpers::send_log,
};

pub async fn resolve_libraries(
    manifest: &UnifiedVersionManifest,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
) -> Result<()> {
    let client = reqwest::Client::new();
    send_log(tx, "Checking libraries...");

    let necessary_libraries = get_necessary_libraries(manifest);
    let total = necessary_libraries.len();
    stream::iter(necessary_libraries.into_iter().enumerate())
        .map(|(index, library)| {
            let tx = tx.clone();
            let client = client.clone();
            let library = library.clone();
            let manifest = manifest.clone();

            async move {
                if index % 100 == 0 {
                    send_log(&tx, format!("Libraries progress: {}/{}", index, total));
                }

                if !check_exist_library(&library, installation_path, &tx) {
                    download_library(&library, installation_path, &tx, &client, &manifest).await
                } else {
                    Ok(())
                }
            }
        })
        .buffer_unordered(64)
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
