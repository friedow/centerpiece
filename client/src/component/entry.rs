pub fn view(ui: &mut egui::Ui, entry: &crate::model::Entry, active: bool) {
    egui::Frame::NONE.show(ui, |ui| {
        ui.set_height(crate::ENTRY_HEIGHT);

        let settings = settings::Settings::get_or_init();
        let stroke_color = if active {
            settings::hexcolor(&settings.color.text)
        } else {
            egui::Color32::TRANSPARENT
        };

        egui::Frame::new()
            .stroke(egui::Stroke::new(1., stroke_color))
            .corner_radius(0.1 * crate::REM)
            .inner_margin(0.5 * crate::REM)
            .outer_margin(egui::vec2(1. * crate::REM, 0.))
            .show(ui, |ui| {
                egui::containers::Sides::new().show(
                    ui,
                    |ui| {
                        ui.set_max_width(700.);
                        ui.add(egui::Label::new(entry.title.clone()).truncate());
                    },
                    |ui| {
                        if active {
                            ui.label(entry.action.clone());
                        }
                    },
                );
            });
    });
}
