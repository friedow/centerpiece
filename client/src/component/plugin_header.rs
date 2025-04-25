pub fn view(plugin: &crate::model::Plugin) -> iced::Element<'static, crate::Message> {
    iced::widget::row![iced::widget::text(plugin.title.clone())
        .font(iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Light,
            ..Default::default()
        })
        .size(0.75 * crate::rem())]
    // We're fixing the height here to unify it
    // with the height of entries for a smooth
    // scrolling experience
    .height(crate::entry_height())
    .padding(iced::Padding {
        top: 0.8 * crate::rem(),
        right: 1.25 * crate::rem(),
        bottom: 0.5 * crate::rem(),
        left: 1.25 * crate::rem(),
    })
    .into()
}
