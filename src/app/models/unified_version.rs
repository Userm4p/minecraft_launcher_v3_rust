use serde::{Deserialize, Serialize};

use crate::app::models::{
    version::{LibraryDownloads, ServerClass, VersionManifest},
    versions_below_26x::VersionManifestBelow26x,
};

pub const CURRENT_OS: &str = if cfg!(target_os = "windows") {
    "windows"
} else if cfg!(target_os = "macos") {
    "osx"
} else if cfg!(target_os = "linux") {
    "linux"
} else {
    "unknown"
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedVersionManifest {
    pub id: String,
    pub downloads: UnifiedDownloads,
    pub libraries: Vec<UnifiedLibrary>,
    pub asset_index: UnifiedAssetIndex,
    pub assets: String,
    pub arguments: UnifiedArguments,

    pub main_class: String,
}

impl UnifiedVersionManifest {
    pub fn from_version_manifest(version_manifest: VersionManifest) -> Self {
        UnifiedVersionManifest {
            id: version_manifest.id,
            downloads: UnifiedDownloads {
                client: version_manifest.downloads.client,
            },
            libraries: version_manifest
                .libraries
                .iter()
                .map(|lib| UnifiedLibrary {
                    name: lib.name.clone(),
                    downloads: LibraryDownloads {
                        artifact: ServerClass {
                            path: lib.downloads.artifact.path.clone(),
                            sha1: lib.downloads.artifact.sha1.clone(),
                            size: lib.downloads.artifact.size,
                            url: lib.downloads.artifact.url.clone(),
                        },
                    },
                    rules: lib.rules.as_ref().map(|rules| {
                        rules
                            .iter()
                            .map(|rule| UnifiedRules {
                                action: rule.action.clone(),
                                os: rule.os.clone(),
                            })
                            .collect()
                    }),
                })
                .collect(),
            asset_index: UnifiedAssetIndex {
                id: version_manifest.asset_index.id.clone(),
                sha1: version_manifest.asset_index.sha1.clone(),
                size: version_manifest.asset_index.size,
                total_size: version_manifest.asset_index.total_size,
                url: version_manifest.asset_index.url.clone(),
            },
            assets: version_manifest.assets.clone(),
            arguments: version_manifest.arguments.clone(),
            main_class: version_manifest.main_class,
        }
    }

    pub fn from_version_manifest_below_26x(version_manifest: VersionManifestBelow26x) -> Self {
        UnifiedVersionManifest {
            id: version_manifest.id,
            downloads: UnifiedDownloads {
                client: ServerClass {
                    path: version_manifest.downloads.client.path.clone(),
                    sha1: version_manifest.downloads.client.sha1.clone(),
                    size: version_manifest.downloads.client.size,
                    url: version_manifest.downloads.client.url.clone(),
                },
            },
            libraries: version_manifest
                .libraries
                .iter()
                .map(|lib| UnifiedLibrary {
                    name: lib.name.clone(),
                    downloads: LibraryDownloads {
                        artifact: ServerClass {
                            path: lib.downloads.artifact.path.clone(),
                            sha1: lib.downloads.artifact.sha1.clone(),
                            size: lib.downloads.artifact.size,
                            url: lib.downloads.artifact.url.clone(),
                        },
                    },
                    rules: lib.rules.as_ref().map(|rules| {
                        rules
                            .iter()
                            .map(|rule| UnifiedRules {
                                action: rule.action.clone(),
                                os: rule.os.clone(),
                            })
                            .collect()
                    }),
                })
                .collect(),
            asset_index: UnifiedAssetIndex {
                id: version_manifest.asset_index.id.clone(),
                sha1: version_manifest.asset_index.sha1.clone(),
                size: version_manifest.asset_index.size,
                total_size: version_manifest.asset_index.total_size,
                url: version_manifest.asset_index.url.clone(),
            },
            assets: version_manifest.assets.clone(),
            arguments: UnifiedArguments {
                default_user_jvm: vec![],
                game: version_manifest
                    .arguments
                    .game
                    .iter()
                    .map(|game_arg| match game_arg {
                        GameElement::GameClass(game_class) => GameElement::GameClass(GameClass {
                            rules: game_class
                                .rules
                                .iter()
                                .map(|rule| GameRule {
                                    action: rule.action.clone(),
                                    features: rule.features.clone(),
                                })
                                .collect(),
                            value: game_class.value.clone(),
                        }),
                        GameElement::String(s) => GameElement::String(s.clone()),
                    })
                    .collect(),
                jvm: version_manifest
                    .arguments
                    .jvm
                    .iter()
                    .map(|jvm_arg| match jvm_arg {
                        JvmElement::JvmClass(jvm_class) => JvmElement::JvmClass(JvmClass {
                            rules: jvm_class
                                .rules
                                .iter()
                                .map(|rule| JvmRule {
                                    action: rule.action.clone(),
                                    os: rule.os.clone(),
                                })
                                .collect(),
                            value: jvm_class.value.clone(),
                        }),
                        JvmElement::String(s) => JvmElement::String(s.clone()),
                    })
                    .collect(),
            },
            main_class: version_manifest.main_class,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub total_size: Option<i64>,
    pub url: String,
}

impl UnifiedRules {
    pub fn applies_to_current_os(&self) -> bool {
        match &self.os {
            TentacledOs { name } => {
                let os_name = match name {
                    Name::Windows => "windows",
                    Name::Osx => "osx",
                    Name::Linux => "linux",
                };
                os_name == CURRENT_OS
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Name {
    Linux,
    Osx,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedRules {
    action: Action,
    os: TentacledOs,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedLibrary {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<UnifiedRules>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct UnifiedDownloads {
    pub client: ServerClass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledOs {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Allow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchContext {
    pub ram_mb: u32,
    pub natives_directory: String,
    pub launcher_name: String,
    pub launcher_version: String,
    pub classpath: String,
    pub java_version: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JvmElement {
    JvmClass(JvmClass),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameElement {
    GameClass(GameClass),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameClass {
    pub rules: Vec<GameRule>,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameRule {
    pub action: Action,
    pub features: Features,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub has_quick_plays_support: Option<bool>,
    pub is_quick_play_singleplayer: Option<bool>,
    pub is_quick_play_multiplayer: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JvmClass {
    pub rules: Vec<JvmRule>,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JvmRule {
    pub action: Action,
    pub os: FluffyOs,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyOs {
    pub name: Option<Name>,
    pub arch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultUserJvm {
    pub value: Vec<String>,
    pub rules: Option<Vec<DefaultUserJvmRule>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultUserJvmRule {
    pub action: Action,
    pub os: PurpleOs,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleOs {
    pub name: Name,
    pub version_range: Option<VersionRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionRange {
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UnifiedArguments {
    pub default_user_jvm: Vec<DefaultUserJvm>,
    pub game: Vec<GameElement>,
    pub jvm: Vec<JvmElement>,
}

pub struct GameLaunchContext {
    pub username: String,
    pub version_name: String,
    pub game_directory: String,
    pub assets_root: String,
    pub assets_index_name: String,
}
