pub fn view(ui: &mut eframe::egui::Ui, query: &mut String, add_horizontal_rule: bool) {
    ui.text_edit_singleline(query);
    if add_horizontal_rule {
        ui.separator();
    }
}
