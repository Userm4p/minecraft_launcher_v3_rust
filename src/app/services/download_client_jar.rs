use std::path::Path;

use crate::app::models::unified_version::UnifiedVersionManifest;

pub async fn download_client_jar(
    version_manifest: &UnifiedVersionManifest,
    installation_path: &str,
) -> anyhow::Result<()> {
    let client_jar_url = version_manifest.downloads.client.url.clone();

    let response = reqwest::get(&client_jar_url).await?;
    let bytes = response.bytes().await?;

    let client_jar_path = Path::new(installation_path)
        .join("versions")
        .join(&version_manifest.id)
        .join(format!("{}.jar", version_manifest.id));

    if let Some(parent) = client_jar_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(client_jar_path, bytes)?;

    Ok(())
}
