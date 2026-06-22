use crate::{
    MyApp,
    app::{ui::widgets::text_input, utils::consts::screens::Screens},
};
use egui::{Color32, FontFamily, FontId, RichText};

pub fn render(ui: &mut egui::Ui, app: &mut MyApp) {
    let has_username = !app.app_state.launcher_settings.username.is_empty();

    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new("MINECRAFT")
                .color(Color32::WHITE)
                .font(FontId {
                    size: 72.0,
                    family: FontFamily::Name(("open_sans_bold").into()),
                }),
        );
        ui.label(
            RichText::new("Launcher")
                .color(Color32::from_rgb(123, 241, 168))
                .font(FontId {
                    size: 20.0,
                    family: FontFamily::Name(("inter_regular").into()),
                }),
        );

        text_input(
            ui,
            "Username",
            &mut app.app_state.launcher_settings.username,
            "",
            250.0,
            true,
        );
        ui.add_space(8.0);
        if ui
            .add_enabled(
                has_username,
                egui::Button::new(
                    egui::RichText::new("Jugar")
                        .font(FontId {
                            size: 20.0,
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
            app.app_state.ui_state.current_screen = Screens::Launch;
        }
    });
}
