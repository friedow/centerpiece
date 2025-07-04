pub fn view(ui: &mut eframe::egui::Ui, entry: &crate::model::Entry, active: bool) {
    let settings = settings::Settings::get_or_init();
    let text_color = settings::hexcolor(&settings.color.text);
    let stroke_color = if active {
        text_color
    } else {
        eframe::egui::Color32::TRANSPARENT
    };
    let stroke = eframe::egui::Stroke::new(1., stroke_color);

    eframe::egui::Frame::new()
        .stroke(stroke)
        .corner_radius(0.1 * crate::REM)
        .inner_margin(0.5 * crate::REM)
        .show(ui, |ui| {
            eframe::egui::containers::Sides::new().show(
                ui,
                |ui| {
                    let mut job = eframe::egui::text::LayoutJob::single_section(
                        entry.title.clone(),
                        eframe::egui::TextFormat {
                            color: text_color,
                            ..Default::default()
                        },
                    );
                    job.wrap = eframe::egui::text::TextWrapping::truncate_at_width(60.);
                    ui.label(job);
                },
                |ui| {
                    if active {
                        ui.label(
                            eframe::egui::RichText::new(entry.action.clone()).color(text_color),
                        );
                    }
                },
            );
        });
}
