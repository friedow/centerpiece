pub const REM: f32 = 14.0;

pub struct Sandbox {}

impl iced::application::StyleSheet for Sandbox {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: iced::Color::WHITE,
        }
    }
}

pub struct ApplicationWrapper {}

impl iced::widget::container::StyleSheet for ApplicationWrapper {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::Color::BLACK)),
            border_color: iced::Color::TRANSPARENT,
            border_radius: iced::BorderRadius::from(0.25 * REM),
            border_width: 0.,
            text_color: None,
        };
    }
}
