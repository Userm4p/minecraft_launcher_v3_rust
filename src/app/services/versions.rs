use anyhow::Result;

use crate::app::models::versions::MinecraftManifestResponse;

pub async fn fetch_versions() -> Result<MinecraftManifestResponse> {
    let response: MinecraftManifestResponse =
        reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await?
            .json::<MinecraftManifestResponse>()
            .await?;
    let versions = response;

    Ok(versions)
}
