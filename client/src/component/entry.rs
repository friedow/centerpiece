pub fn view(ui: &mut eframe::egui::Ui, entry: &crate::model::Entry, active: bool) {
    let settings = settings::Settings::get_or_init();
    let stroke_color = if active {
        settings::hexcolor(&settings.color.text)
    } else {
        eframe::egui::Color32::TRANSPARENT
    };
    let stroke = eframe::egui::Stroke::new(1., stroke_color);

    eframe::egui::Frame::new()
        .stroke(stroke)
        .corner_radius(0.1 * crate::REM)
        .inner_margin(0.5 * crate::REM)
        .outer_margin(eframe::egui::vec2(1. * crate::REM, 0.))
        .show(ui, |ui| {
            eframe::egui::containers::Sides::new().show(
                ui,
                |ui| {
                    ui.add(eframe::egui::Label::new(entry.title.clone()).truncate());
                },
                |ui| {
                    if active {
                        ui.label(entry.action.clone());
                    }
                },
            );
        });
}
