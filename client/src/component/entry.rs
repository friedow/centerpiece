pub fn view(entry: &crate::model::Entry, active: bool) -> iced::Element<'static, crate::Message> {
    iced::widget::container(
        iced::widget::row![
            iced::widget::text(&entry.title)
                .size(1. * crate::REM)
                .width(iced::Length::Fill),
            iced::widget::text(if active { &entry.action } else { "" }).size(1. * crate::REM)
        ]
        .padding(0.5 * crate::REM),
    )
    .style(style(active))
    .into()
}

fn style(active: bool) -> iced::theme::Container {
    if active {
        iced::theme::Container::Custom(Box::new(Style {}))
    } else {
        iced::theme::Container::Transparent
    }
}

pub struct Style {}

impl iced::widget::container::StyleSheet for Style {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: None,
            text_color: None,
            border: iced::Border {
                radius: iced::border::Radius::from(0.1 * crate::REM),
                width: 1.,
                color: iced::Color::WHITE,
            },
            shadow: iced::Shadow::default(),
        }
    }
}
