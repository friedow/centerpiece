pub fn view(plugin: &crate::model::Plugin) -> iced::Element<'static, crate::Message> {
    iced::widget::row![iced::widget::text(plugin.title.clone())
        .font(iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Light,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::default(),
        })
        .size(0.75 * crate::REM)]
    // We're fixing the height here to unify it
    // with the height of entries for a smooth
    // scrolling experience
    .height(crate::ENTRY_HEIGHT)
    .padding(iced::Padding {
        top: 0.8 * crate::REM,
        right: 1.25 * crate::REM,
        bottom: 0.5 * crate::REM,
        left: 1.25 * crate::REM,
    })
    .into()
}
