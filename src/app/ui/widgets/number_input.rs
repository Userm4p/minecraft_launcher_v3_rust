use egui::{Color32, DragValue, FontFamily, FontId, RichText, Stroke};

pub fn number_input(ui: &mut egui::Ui, label: &str, value: &mut u32, width: f32, suffix: &str) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new(label)
                .font(FontId {
                    size: 14.0,

                    family: FontFamily::Name(("inter_18pt_medium").into()),
                })
                .color(Color32::WHITE),
        );

        egui::Frame::default()
            .fill(Color32::from_rgb(20, 20, 20))
            .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
            .corner_radius(10.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.set_width(width);

                ui.add(
                    DragValue::new(value)
                        .speed(128)
                        .range(512..=32768)
                        .suffix(suffix),
                );
            });
    });
}
