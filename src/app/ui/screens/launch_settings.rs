use egui::{CentralPanel, FontFamily, FontId, RichText};

use crate::{
    MyApp,
    app::ui::widgets::{number_input, styled_checkbox},
};

const WIDTH: f32 = 300.0;

pub fn render(ui: &mut egui::Ui, app: &mut MyApp) {
    CentralPanel::default()
        .frame(egui::Frame::default().inner_margin(16))
        .show_inside(ui, |ui| {
            ui.vertical(|ui| {
                header(ui);
                settings_form(ui, app);
            });
        });
}

fn header(ui: &mut egui::Ui) {
    ui.heading(
        RichText::new("Launcher Settings")
            .font(FontId {
                size: 24.0,
                family: FontFamily::Name(("inter_24pt_semibold").into()),
            })
            .color(egui::Color32::WHITE),
    );
}

fn settings_form(ui: &mut egui::Ui, app: &mut MyApp) {
    number_input(
        ui,
        "Memoria ram",
        &mut app.app_state.launcher_settings.ram_allocation,
        WIDTH,
        " MB",
    );
    styled_checkbox(
        ui,
        &mut app.app_state.launcher_settings.show_snapshot_versions,
        "Mostrar versiones snapshot",
    );
}
