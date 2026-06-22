use crate::app::models::versions::Latest;
use crate::app::models::versions::MinecraftManifestResponse;
use crate::app::utils::consts::screens::Screens;

pub struct VersionsState {
    pub loading: bool,
    pub minecraft_manifest: MinecraftManifestResponse,
    pub error: Option<String>,
}

pub struct LauncherSettings {
    pub selected_version: String,
    pub username: String,
    pub ram_allocation: u32,
    pub installation_path: String,
    pub show_snapshot_versions: bool,
}

pub enum LauncherCommand {
    Launch {
        version_url: String,
        username: String,
        ram_allocation: u32,
        installation_path: String,
    },
}

pub enum LauncherEvent {
    Log(String),
    Error(String),
    Progress(f32),
    Finished,
    Started,
}

pub struct UiState {
    pub current_screen: Screens,
    pub launch_in_progress: bool,
    pub download_progress: f32,
    pub error_message: Option<String>,
    pub logs: Vec<String>,
}

pub struct AppState {
    pub ui_state: UiState,
    pub version_state: VersionsState,
    pub launcher_settings: LauncherSettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            ui_state: UiState {
                current_screen: Screens::Home,
                launch_in_progress: false,
                download_progress: 0.0,
                error_message: None,
                logs: Vec::new(),
            },
            version_state: VersionsState {
                loading: true,
                minecraft_manifest: MinecraftManifestResponse {
                    versions: Vec::new(),
                    latest: Latest {
                        release: String::new(),
                        snapshot: String::new(),
                    },
                },
                error: None,
            },
            launcher_settings: LauncherSettings {
                selected_version: String::new(),
                username: String::new(),
                ram_allocation: 2048,
                installation_path: String::from(".minecraft"),
                show_snapshot_versions: false,
            },
        }
    }
}
