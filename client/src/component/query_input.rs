pub fn view(ui: &mut egui::Ui, query: &mut String) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(query)
            .hint_text("Search")
            .lock_focus(true)
            .desired_width(f32::INFINITY)
            .frame(false)
            .margin(egui::epaint::Marginf {
                left: 1. * crate::REM,
                right: 1. * crate::REM,
                top: 1. * crate::REM,
                bottom: 0.75 * crate::REM,
            }),
    )
}
