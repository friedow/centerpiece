pub fn view(ui: &mut eframe::egui::Ui, plugin: &crate::model::Plugin) {
    eframe::egui::Frame::none()
        .outer_margin(eframe::egui::epaint::Marginf {
            left: 1.25 * crate::REM,
            right: 1.25 * crate::REM,
            top: 1. * crate::REM,
            bottom: 0.5 * crate::REM,
        })
        .show(ui, |ui| {
            ui.heading(plugin.title.clone());
        });
}
