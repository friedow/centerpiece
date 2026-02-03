use crate::settings;

pub fn view(plugin: &crate::model::Plugin) -> iced::Element<'static, crate::Message> {
    let mut font = settings().default_font.clone();
    font.weight = iced::font::Weight::Light;

    iced::widget::row![iced::widget::text(plugin.title.clone())
        .font(font)
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
