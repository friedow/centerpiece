pub fn view() -> iced::Element<'static, crate::Message> {
    iced::widget::column![iced::widget::horizontal_rule(1)]
        .padding(iced::Padding::from([
            1. * crate::REM,
            0.,
            0.5 * crate::REM,
            0.,
        ]))
        // We're fixing the height here to unitfy it
        // with the height of entries for a smooth
        // scrolling experience
        .height(crate::ENTRY_HEIGHT)
        .into()
}
