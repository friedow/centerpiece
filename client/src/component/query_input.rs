pub const SEARCH_INPUT_ID: &str = "search_input";

pub fn view(query: &str, add_horizontal_rule: bool) -> iced::Element<'static, crate::Message> {
    let mut view = iced::widget::column![iced::widget::text_input("Search", query)
        .id(SEARCH_INPUT_ID)
        .on_input(crate::Message::Search)
        .icon(iced::widget::text_input::Icon {
            font: crate::settings().default_font,
            code_point: '󰍉',
            size: Some(iced::Pixels(1.3 * crate::REM)),
            spacing: crate::REM,
            side: iced::widget::text_input::Side::Left,
        })
        .size(1. * crate::REM)
        .padding([1.0 * crate::REM, 1.2 * crate::REM])
        .style(style),]
    .padding(iced::Padding::default().bottom(1.));

    if add_horizontal_rule {
        view = view.push(iced::widget::horizontal_rule(1));
    }

    view.into()
}

pub fn style(
    theme: &iced::Theme,
    _status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let palette = theme.extended_palette();

    iced::widget::text_input::Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border::default(),
        icon: palette.background.weak.text,
        placeholder: palette.background.strong.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    }
}
