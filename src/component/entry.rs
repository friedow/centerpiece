use crate::model;
use crate::style;
use crate::Message;

pub fn view(entry: &model::Entry, active: bool) -> iced::Element<'static, Message> {
    return iced::widget::container(
        iced::widget::row![
            iced::widget::text(&entry.title)
                .size(1. * style::REM)
                .width(iced::Length::Fill),
            iced::widget::text(&entry.action).size(1. * style::REM),
        ]
        .padding(0.5 * style::REM),
    )
    .style(style(active))
    .into();
}

fn style(active: bool) -> iced::theme::Container {
    if active {
        return iced::theme::Container::Custom(Box::new(Style {}));
    } else {
        return iced::theme::Container::Transparent;
    }
}

pub struct Style {}

impl iced::widget::container::StyleSheet for Style {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: None,
            border_radius: 0.1 * style::REM,
            border_width: 1.,
            border_color: iced::color!(0xffffff, 1.),
            text_color: None,
        };
    }
}
