use egui::{Color32, FontFamily, FontId, Frame, Margin, Panel, RichText, Stroke, Ui};

use crate::app::utils::consts::screens::Screens;
const SELECTED_COLOR: Color32 = Color32::from_rgb(0, 166, 62);
const UNSELECTED_COLOR: Color32 = Color32::TRANSPARENT;

pub fn render(ui: &mut Ui, current_screen: &mut Screens) {
    Panel::left("sidebar")
        .exact_size(250.0)
        .show_inside(ui, |ui| {
            Frame::default()
                .inner_margin(Margin {
                    left: 12,
                    top: 8,
                    ..Margin::default()
                })
                .show(ui, |ui| {
                    header(ui);
                    sidebar_content(ui, current_screen);
                    footer(ui);
                });
        });
}

fn header(ui: &mut Ui) {
    ui.label(
        RichText::new("MINECRAFT")
            .color(Color32::WHITE)
            .font(FontId {
                size: 24.0,
                family: FontFamily::Name(("open_sans_bold").into()),
            }),
    );
    ui.label(
        RichText::new("Launcher")
            .color(Color32::from_rgb(123, 241, 168))
            .font(FontId {
                size: 16.0,
                family: FontFamily::Name(("open_sans_bold").into()),
            }),
    );

    ui.add_space(12.0);
}

fn sidebar_content(ui: &mut Ui, current_screen: &mut Screens) {
    if ui
        .add(
            egui::Button::new(
                egui::RichText::new("Jugar")
                    .font(FontId {
                        size: 16.0,
                        family: FontFamily::Name(("inter_18pt_semibold").into()),
                    })
                    .color(egui::Color32::WHITE),
            )
            .fill(if *current_screen == Screens::Launch {
                SELECTED_COLOR
            } else {
                UNSELECTED_COLOR
            })
            .corner_radius(10.0)
            .min_size(egui::vec2(200.0, 50.0)),
        )
        .clicked()
    {
        *current_screen = Screens::Launch;
    }

    ui.add_space(8.0);

    if ui
        .add(
            egui::Button::new(
                egui::RichText::new("Configuración")
                    .font(FontId {
                        size: 16.0,
                        family: FontFamily::Name(("inter_18pt_semibold").into()),
                    })
                    .color(egui::Color32::WHITE),
            )
            .fill(if *current_screen == Screens::LaunchSettings {
                SELECTED_COLOR
            } else {
                UNSELECTED_COLOR
            })
            .corner_radius(10.0)
            .min_size(egui::vec2(200.0, 50.0)),
        )
        .clicked()
    {
        *current_screen = Screens::LaunchSettings;
    }
}

fn footer(ui: &mut Ui) {
    ui.add_space(ui.available_height() - 96.0);
    ui.separator();

    let exit_button = egui::Button::new(egui::RichText::new("Salir").font(FontId {
        size: 16.0,
        family: FontFamily::Name(("inter_18pt_semibold").into()),
    }))
    .stroke(Stroke::NONE)
    .fill(Color32::TRANSPARENT)
    .corner_radius(10.0)
    .min_size(egui::vec2(200.0, 50.0));

    let response = ui.add(exit_button);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.clicked() {
        std::process::exit(0);
    }
    ui.label(
        RichText::new("Minecraft Launcher v3")
            .color(Color32::WHITE)
            .font(FontId {
                size: 12.0,
                family: FontFamily::Name(("open_sans_bold").into()),
            }),
    );
}
