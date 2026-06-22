use crate::{
    MyApp,
    app::{
        app_state::{LauncherCommand, VersionsState},
        models::versions::VersionType,
    },
};
use egui::{CentralPanel, Color32, FontFamily, FontId, Frame, RichText};

pub fn render(ui: &mut egui::Ui, app: &mut MyApp) {
    CentralPanel::default()
        .frame(egui::Frame::default().inner_margin(16))
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                logs_container(ui, &app.app_state.ui_state.logs);
                title(ui);
                versions_selector(ui, app);
                play_button(ui, app);
            });
        });
}

fn title(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("Jugar Minecraft")
            .font(FontId {
                size: 24.0,
                family: FontFamily::Name(("inter_24pt_semibold").into()),
            })
            .color(egui::Color32::WHITE),
    );
}

fn versions_selector(ui: &mut egui::Ui, app: &mut MyApp) {
    let version_state: &VersionsState = &app.app_state.version_state;

    let versions = version_state
        .minecraft_manifest
        .versions
        .iter()
        .filter(|v| {
            if app.app_state.launcher_settings.show_snapshot_versions {
                true
            } else {
                v.version_type == VersionType::Release
            }
        });
    ui.vertical_centered(|ui| {
        ui.label("Version");

        ui.horizontal(|ui| {
            let available = ui.available_width();

            let combo_width = 250.0;

            let left_space = (available - combo_width) / 2.0;

            if left_space > 0.0 {
                ui.add_space(left_space);
            }

            egui::ComboBox::from_id_salt("version_selector")
                .width(combo_width)
                .selected_text(&app.app_state.launcher_settings.selected_version)
                .show_ui(ui, |ui| {
                    ui.set_min_width(combo_width);

                    for version in versions {
                        ui.selectable_value(
                            &mut app.app_state.launcher_settings.selected_version,
                            version.id.clone(),
                            &version.id,
                        );
                    }
                });
        });
    });
}
fn logs_container(ui: &mut egui::Ui, logs: &[String]) {
    ui.label(
        RichText::new("Logs:")
            .font(FontId {
                size: 18.0,
                family: FontFamily::Name(("inter_18pt_medium").into()),
            })
            .color(Color32::WHITE),
    );

    let available_height = (ui.available_height() - 250.0).max(200.0);

    Frame::default().fill(Color32::DARK_GRAY).show(ui, |ui| {
        ui.set_width(ui.available_width());

        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height(available_height)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(ui.available_width(), available_height));
                let visible_logs = logs
                    .iter()
                    .rev()
                    .take(200)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>();
                ui.vertical(|ui| {
                    for log in visible_logs {
                        ui.label(
                            RichText::new(format!("[INFO] {}", log)).color(Color32::LIGHT_GREEN),
                        );
                    }
                });
            });
    });
}

fn play_button(ui: &mut egui::Ui, app: &mut MyApp) {
    if ui
        .add_enabled(
            !app.app_state.ui_state.launch_in_progress,
            egui::Button::new(
                egui::RichText::new("Empezar a jugar")
                    .font(FontId {
                        size: 16.0,
                        family: FontFamily::Name(("inter_18pt_semibold").into()),
                    })
                    .color(egui::Color32::WHITE),
            )
            .fill(egui::Color32::from_rgb(0, 166, 62))
            .corner_radius(10.0)
            .min_size(egui::vec2(200.0, 50.0)),
        )
        .clicked()
    {
        let selected_version = app
            .app_state
            .version_state
            .minecraft_manifest
            .versions
            .iter()
            .find(|v| v.id == app.app_state.launcher_settings.selected_version);
        let username = app.app_state.launcher_settings.username.clone();
        let ram_allocation = app.app_state.launcher_settings.ram_allocation;
        let installation_path = app.app_state.launcher_settings.installation_path.clone();
        match selected_version {
            Some(version) => {
                let _ = app.launcher_tx.send(LauncherCommand::Launch {
                    version_url: version.url.clone(),
                    username,
                    ram_allocation,
                    installation_path,
                });
            }

            None => {
                app.app_state
                    .ui_state
                    .logs
                    .push("Selected version not found".into());
            }
        }
    }
}
