use std::path::PathBuf;

use anyhow::Result;

use crate::app::models::unified_version::UnifiedVersionManifest;

pub fn build_classpath(
    manifest: &UnifiedVersionManifest,
    installation_path: &str,
) -> Result<String> {
    let separator = if cfg!(windows) { ";" } else { ":" };

    let mut classpath_entries: Vec<String> = Vec::new();

    for library in &manifest.libraries {
        let Some(path) = &library.downloads.artifact.path else {
            continue;
        };

        let library_path = PathBuf::from(installation_path)
            .join("libraries")
            .join(path);

        if library_path.exists() {
            classpath_entries.push(
                library_path.to_string_lossy().to_string(),
            );
        }
    }

    let client_jar = PathBuf::from(installation_path)
        .join("versions")
        .join(&manifest.id)
        .join(format!("{}.jar", manifest.id));

    classpath_entries.push(
        client_jar.to_string_lossy().to_string(),
    );

    Ok(classpath_entries.join(separator))
}