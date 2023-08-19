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

pub struct TextInput {}

impl iced::widget::text_input::StyleSheet for TextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        return iced::widget::text_input::Appearance {
            background: iced::Background::Color(iced::color!(0x000000, 0.)),
            border_radius: 0.,
            border_width: 0.,
            border_color: iced::color!(0x000000, 0.),
            icon_color: iced::color!(0xf3f3f3, 1.),
        };
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        return self.active(style);
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        return self.active(style);
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xf3f3f3, 1.);
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xffffff, 1.);
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xfafafa, 1.);
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0x1b1b1b, 1.);
    }
}

pub struct ActiveEntry {}

impl iced::widget::container::StyleSheet for ActiveEntry {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: None,
            border_radius: 0.1 * REM,
            border_width: 1.,
            border_color: iced::color!(0xffffff, 1.),
            text_color: None,
        };
    }
}
