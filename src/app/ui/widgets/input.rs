use egui::{Color32, FontFamily, FontId, RichText, Stroke, TextEdit};

pub fn text_input(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    width: f32,
    center: bool,
) {
    if center {
        ui.vertical_centered(|ui| {
            base_input(ui, label, value, hint, width);
        });
    } else {
        ui.vertical(|ui| {
            base_input(ui, label, value, hint, width);
        });
    }
}

fn base_input(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str, width: f32) {
    ui.label(
        RichText::new(label)
            .font(FontId {
                size: 14.0,

                family: FontFamily::Name(("inter_18pt_medium").into()),
            })
            .color(Color32::WHITE),
    );

    egui::Frame::default()
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0, Color32::TRANSPARENT))
        .corner_radius(10.0)
        .inner_margin(2.0)
        .show(ui, |ui| {
            ui.add(
                TextEdit::singleline(value)
                    .hint_text(hint)
                    .desired_width(width)
                    .font(FontId {
                        size: 16.0,

                        family: FontFamily::Name(("inter_18pt_regular").into()),
                    })
                    .margin(egui::vec2(12.0, 10.0))
                    .horizontal_align(egui::Align::Center),
            );
        });
}
