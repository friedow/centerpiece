pub fn view(ui: &mut eframe::egui::Ui, query: &mut String) -> eframe::egui::Response {
    return ui.add(
        eframe::egui::TextEdit::singleline(query)
            .hint_text("Search")
            .lock_focus(true)
            .desired_width(f32::INFINITY)
            .frame(false)
            .margin(eframe::egui::epaint::Marginf {
                left: 1. * crate::REM,
                right: 1. * crate::REM,
                top: 1. * crate::REM,
                bottom: 0.75 * crate::REM,
            }),
    );
}
