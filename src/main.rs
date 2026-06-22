use anyhow::Result;

use eframe::egui;
use egui::{CentralPanel, Color32};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::app::{
    app_state::{AppState, LauncherCommand, LauncherEvent},
    models::versions::MinecraftManifestResponse,
    services::fetch_versions,
    ui::{
        screens::{render_home, render_launch, render_launch_settings},
        widgets::render_sidebar,
    },
    utils::{consts::screens::Screens, fonts::setup_custom_fonts::setup_custom_fonts},
    workers::{
        launch_commands_manager::launch_commands_manager, launch_events_manager,
        manage_versions_event,
    },
};
mod app;

fn main() {
    let options = eframe::NativeOptions::default();

    match eframe::run_native(
        "Minecraft Launcher v3",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    ) {
        Ok(result) => result,
        Err(_e) => {
            print!("{}", String::from("Error al iniciar la aplicación"));
        }
    }
}

pub struct MyApp {
    app_state: AppState,
    runtime: tokio::runtime::Runtime,
    versions_rx: mpsc::UnboundedReceiver<Result<MinecraftManifestResponse>>,
    pub launcher_tx: UnboundedSender<LauncherCommand>,
    pub launcher_rx: UnboundedReceiver<LauncherEvent>,
}

impl Default for MyApp {
    fn default() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let app_state = AppState::default();
        let (versions_tx, versions_rx) = mpsc::unbounded_channel();
        let (launcher_tx, mut launcher_command_rx) = mpsc::unbounded_channel();

        let (launcher_event_tx, launcher_rx) = mpsc::unbounded_channel();

        runtime.spawn(async move {
            let result = fetch_versions().await;
            let _ = versions_tx.send(result);
        });

        runtime.spawn(async move {
            launch_commands_manager(&mut launcher_command_rx, launcher_event_tx).await;
        });
        Self {
            app_state,
            runtime,
            versions_rx,
            launcher_rx,
            launcher_tx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        manage_versions_event(self);
        launch_events_manager(self);

        ctx.request_repaint();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.style_mut().spacing.button_padding = egui::vec2(8.0, 8.0);
        match self.app_state.ui_state.current_screen {
            Screens::Home => {
                CentralPanel::default()
                    .frame(
                        egui::Frame::default()
                            .inner_margin(120)
                            .fill(Color32::from_rgb(3, 46, 21)),
                    )
                    .show_inside(ui, |ui| render_home(ui, self));
            }
            _ => {
                render_sidebar(ui, &mut self.app_state.ui_state.current_screen);

                match &self.app_state.ui_state.current_screen {
                    Screens::Launch => {
                        render_launch(ui, self);
                    }
                    Screens::LaunchSettings => {
                        render_launch_settings(ui, self);
                    }
                    _ => {
                        ui.label("Pantalla no implementada");
                    }
                }
            }
        }
    }
}
