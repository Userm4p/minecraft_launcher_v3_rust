use egui::*;

pub fn styled_checkbox(ui: &mut Ui, value: &mut bool, text: &str) -> Response {
    ui.add(Checkbox::new(
        value,
        RichText::new(text)
            .size(16.0)
            .color(Color32::WHITE)
            .family(FontFamily::Name("inter_regular".into())),
    ))
}
