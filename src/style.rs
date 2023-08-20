pub const REM: f32 = 14.0;

pub struct Sandbox {}

impl iced::application::StyleSheet for Sandbox {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::color!(0x000000, 0.),
            text_color: iced::color!(0xffffff, 1.),
        }
    }
}

pub struct ApplicationWrapper {}

impl iced::widget::container::StyleSheet for ApplicationWrapper {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::color!(0x000000, 1.))),
            border_color: iced::color!(0x000000, 0.),
            border_radius: 0.25 * REM,
            border_width: 0.,
            text_color: None,
        };
    }
}
