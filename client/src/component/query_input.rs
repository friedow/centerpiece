pub const SEARCH_INPUT_ID: &str = "search_input";

pub fn view(query: &str) -> iced::widget::Row<'static, crate::Message> {
    iced::widget::row![
        iced::widget::container(iced::widget::text("󰍉 ").size(1.3 * crate::REM)).padding(
            iced::Padding::from([0.2 * crate::REM, -0.3 * crate::REM, 0., 0.])
        ),
        iced::widget::text_input("Search", query)
            .id(iced::widget::text_input::Id::new(SEARCH_INPUT_ID))
            .on_input(crate::Message::Search)
            .size(1. * crate::REM)
            .width(crate::WIDTH)
            .style(style())
    ]
    .padding(iced::Padding::from([0.8 * crate::REM, 1.2 * crate::REM]))
    .into()
}

fn style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(Style {}))
}

pub struct Style {}

impl iced::widget::text_input::StyleSheet for Style {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        use iced::color;
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border {
                radius: iced::border::Radius::from(0.),
                width: 0.,
                color: iced::Color::TRANSPARENT,
            },
            icon_color: iced::color!(0xf3f3f3, 1.),
        }
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        use iced::color;
        iced::color!(0xf3f3f3, 1.)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        use iced::color;
        iced::color!(0xffffff, 1.)
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        use iced::color;
        iced::color!(0xfafafa, 1.)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        use iced::color;
        iced::color!(0x1b1b1b, 1.)
    }
}
