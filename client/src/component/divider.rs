pub fn view() -> iced::Element<'static, crate::Message> {
    iced::widget::column![iced::widget::horizontal_rule(1)]
        .padding(iced::Padding {
            top: 1. * crate::rem(),
            right: 0.,
            bottom: 0.5 * crate::rem(),
            left: 0.,
        })
        // We're fixing the height here to unify it
        // with the height of entries for a smooth
        // scrolling experience
        .height(crate::entry_height())
        .into()
}
