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
    // iced::widget::row![iced::widget::text(plugin.title.clone())
    //     .font(iced::Font {
    //         family: iced::font::Family::Name("FiraCode Nerd Font"),
    //         weight: iced::font::Weight::Light,
    //         ..Default::default()
    //     })
    //     .size(0.75 * crate::REM)]
    // // We're fixing the height here to unify it
    // // with the height of entries for a smooth
    // // scrolling experience
    // .height(crate::ENTRY_HEIGHT)
    // .padding(iced::Padding {
    //     top: 0.8 * crate::REM,
    //     right: 1.25 * crate::REM,
    //     bottom: 0.5 * crate::REM,
    //     left: 1.25 * crate::REM,
    // })
    // .into()
}
