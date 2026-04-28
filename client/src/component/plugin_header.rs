pub fn view(ui: &mut egui::Ui, plugin: &crate::model::Plugin, add_seperator: bool) {
    egui::Frame::NONE.show(ui, |ui| {
        ui.set_height(crate::ENTRY_HEIGHT);
        if add_seperator {
            ui.separator();
        }
        egui::Frame::NONE
            .outer_margin(egui::epaint::MarginF32 {
                left: 1.25 * crate::REM,
                right: 1.25 * crate::REM,
                top: if add_seperator {
                    0.4 * crate::REM
                } else {
                    1. * crate::REM
                },
                bottom: 0.5 * crate::REM,
            })
            .show(ui, |ui| {
                ui.heading(plugin.title.clone());
            });
    });
}
