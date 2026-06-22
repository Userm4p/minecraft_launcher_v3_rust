
pub fn resolve_assets_path(installation_path: &str, hash: &str) -> String {
    let assets_path = format!("{}/assets/objects", installation_path);
    let two_chars = &hash[0..2];
    let asset_path = format!("{}/{}/{}", assets_path, two_chars, hash);
    asset_path
}