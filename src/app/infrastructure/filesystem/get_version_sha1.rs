use sha1::{Digest, Sha1};

pub fn get_version_sha1(installation_path: &str, version_id: &str) -> Option<String> {
    let version_jar_path = std::path::Path::new(installation_path)
        .join("versions")
        .join(version_id)
        .join(format!("{}.jar", version_id));

    if version_jar_path.exists() {
        match std::fs::read(&version_jar_path) {
            Ok(data) => {
                let mut hasher = Sha1::new();
                hasher.update(&data);
                let result = hasher.finalize();
                Some(hex::encode(result))
            }
            Err(_) => None,
        }
    } else {
        None
    }
}
