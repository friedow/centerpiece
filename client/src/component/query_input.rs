pub const SEARCH_INPUT_ID: &str = "search_input";

pub fn view(query: &str, add_horizontal_rule: bool) -> iced::Element<'static, crate::Message> {
    let mut view = iced::widget::column![iced::widget::row![
        iced::widget::container(iced::widget::text("󰍉 ").size(1.3 * crate::REM)).padding([
            0.2 * crate::REM,
            -0.3 * crate::REM,
            0.,
            0.
        ]),
        iced::widget::text_input("Search", query)
            .id(SEARCH_INPUT_ID)
            .on_input(crate::Message::Search)
            .size(1. * crate::REM)
            .style(|theme: &iced::Theme| {
                // TODO: should probably use the theme instead of settings here
                let palette = theme.extended_palette();
                let color_settings = crate::settings::Settings::get_or_init();

                iced::widget::text_input::Style {
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: iced::Border::default(),
                    icon: crate::settings::hexcolor(&color_settings.color.surface),
                    placeholder: crate::settings::hexcolor(&color_settings.color.surface),
                    value: crate::settings::hexcolor(&color_settings.color.surface),
                    selection: crate::settings::hexcolor(&color_settings.color.surface),
                }
            })
    ]
    .padding([0.8 * crate::REM, 1.2 * crate::REM]),]
    .padding([0., 0., 1., 0.]);

    if add_horizontal_rule {
        view = view.push(iced::widget::horizontal_rule(1));
    }

    view.into()
}
