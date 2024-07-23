pub fn view(plugin: &crate::model::Plugin) -> iced::Element<'static, crate::Message> {
    iced::widget::row![iced::widget::text(&plugin.title)
        .font(iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Light,
            stretch: iced::font::Stretch::Normal,
            monospaced: true,
        })
        .size(0.75 * crate::REM)]
    .padding(iced::Padding::from([
        0.8 * crate::REM,
        1.25 * crate::REM,
        0.5 * crate::REM,
        1.25 * crate::REM,
    ]))
    .into()
}
