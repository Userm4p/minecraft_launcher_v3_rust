use std::path::Path;

use anyhow::{Result, Error}; 
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent,
    infrastructure::filesystem::{
        resolve_assets, resolve_client_jar, resolve_libraries, save_assets_index,
        save_version_manifest,
    },
    models::unified_version::{GameLaunchContext, LaunchContext},
    services::{
        assets::{
            check_exist_assets_index::check_exist_assets_index,
            download_assets_index::get_current_assets_index,
        },
        get_current_version_manifest,
        java::{
            build_classpath::build_classpath,
            build_game_arguments::build_game_arguments,
            build_jvm_arguments::build_jvm_arguments,
            launch_game::{self, launch_minecraft},
        },
    },
    utils::helpers::{get_java_version_number, send_log},
};

pub async fn handle_launch(
    installation_path: &str,
    version_url: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
    username: &str,
    ram_allocation_mb: u32,
) -> Result<()> {
    send_log(tx, format!("Fetching version manifest..."));

    let manifest = get_current_version_manifest(version_url).await?;

    send_log(tx, format!("Manifest downloaded: {}", manifest.id));

    save_version_manifest(Path::new(installation_path), &manifest)?;

    send_log(tx, format!("Manifest saved: {}", manifest.id));

    send_log(
        tx,
        format!("Resolving client JAR for version {}...", manifest.id),
    );

    resolve_client_jar(&manifest, installation_path, tx).await?;

    send_log(
        tx,
        format!("Client JAR resolved for version {}", manifest.id),
    );

    send_log(
        tx,
        format!("Resolving libraries for version {}...", manifest.id),
    );

    resolve_libraries(&manifest, installation_path, tx).await?;

    send_log(
        tx,
        format!("Libraries resolved for version {}", manifest.id),
    );

    send_log(
        tx,
        format!("Downloading libraries for version {}...", manifest.id),
    );

    resolve_libraries(&manifest, installation_path, tx).await?;

    send_log(
        tx,
        format!("Libraries downloaded for version {}", manifest.id),
    );

    send_log(
        tx,
        format!("Downloading assets index for version {}...", manifest.id),
    );

    let assets_index = get_current_assets_index(&manifest, installation_path, tx).await?;

    send_log(
        tx,
        format!("Assets index downloaded for version {}", manifest.id),
    );

    if !check_exist_assets_index(&manifest, installation_path, tx) {
        save_assets_index(Path::new(installation_path), &assets_index, &manifest)?;
    };

    send_log(
        tx,
        format!("Assets index saved for version {}", manifest.id),
    );

    send_log(
        tx,
        format!("Resolving assets for version {}...", manifest.id),
    );

    resolve_assets(installation_path, tx, assets_index.clone()).await?;

    send_log(tx, format!("Assets resolved for version {}", manifest.id));

    let class_path = build_classpath(&manifest, installation_path)?;

    send_log(tx, &class_path);

    let java_version_number = get_java_version_number().ok_or(Error::msg("Java version not found"))?;

    let context = LaunchContext {
        ram_mb: ram_allocation_mb,
        classpath: class_path.to_string(),
        launcher_name: "My Rust Launcher".to_string(),
        launcher_version: "1.0.0".to_string(),
        natives_directory: format!("{}/versions/{}/natives", installation_path, manifest.id),
        java_version: java_version_number,
    };

    let jvm_args = build_jvm_arguments(&manifest, &context);

    send_log(tx, format!("JVM Arguments: {:?}", jvm_args));

    let game_launcher_context = GameLaunchContext {
        assets_index_name: manifest.assets.to_string(),
        assets_root: format!("{}/assets", installation_path),
        game_directory: installation_path.to_string(),
        version_name: manifest.id.clone(),
        username: username.to_string(),
    };

    let game_arguments = build_game_arguments(&manifest, &game_launcher_context);

    launch_minecraft("javaw", jvm_args, &manifest.main_class, game_arguments, tx)?;

    Ok(())
}
