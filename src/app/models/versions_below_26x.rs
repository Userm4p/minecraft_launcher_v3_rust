use serde::{Deserialize, Serialize};

use crate::app::models::unified_version::{Action, GameElement, JvmElement, TentacledOs};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifestBelow26x {
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: i64,
    pub downloads: VersionManifestDownloads,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
    pub main_class: String,
    pub minimum_launcher_version: i64,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub version_manifest_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arguments {
    pub game: Vec<GameElement>,
    pub jvm: Vec<JvmElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub total_size: Option<i64>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionManifestDownloads {
    pub client: ClientMappingsClass,
    pub client_mappings: ClientMappingsClass,
    pub server: ClientMappingsClass,
    pub server_mappings: ClientMappingsClass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMappingsClass {
    pub sha1: String,
    pub size: i64,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<LibraryRule>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: ClientMappingsClass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryRule {
    pub action: Action,
    pub os: TentacledOs,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingClient {
    pub argument: String,
    pub file: AssetIndex,
    #[serde(rename = "type")]
    pub client_type: String,
}
