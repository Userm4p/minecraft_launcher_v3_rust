use sha1::{Digest, Sha1};
use std::fs;
use tokio::sync::mpsc;

use crate::app::{
    app_state::LauncherEvent, models::unified_version::UnifiedLibrary, utils::helpers::send_log,
};

pub fn check_exist_library(
    library: &UnifiedLibrary,
    installation_path: &str,
    tx: &mpsc::UnboundedSender<LauncherEvent>,
) -> bool {
    let library_download_path = library.downloads.artifact.path.clone();
    send_log(tx, format!("Checking library: {}", library.name));
    if let Some(library_download_path) = library_download_path {
        let full_path = format!("{}/libraries/{}", installation_path, library_download_path);

        if std::path::Path::new(&full_path).exists() {
            match fs::read(&full_path) {
                Ok(data) => {
                    let mut hasher = Sha1::new();
                    hasher.update(&data);
                    let result = hasher.finalize();
                    let file_sha1 = hex::encode(result);
                    file_sha1 == library.downloads.artifact.sha1
                }
                Err(_) => false,
            }
        } else {
            send_log(tx, format!("library: {} does not exist", library.name));
            return false;
        }
    } else {
        send_log(tx, format!("library: {} does not exist", library.name));
        false
    }
}
