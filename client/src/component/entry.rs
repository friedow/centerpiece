pub fn view(entry: &crate::model::Entry, active: bool) -> iced::Element<'static, crate::Message> {
    return iced::widget::container(
        iced::widget::container(
            iced::widget::row![
                iced::widget::text(clipped_title(entry.title.clone()))
                    .size(1. * crate::REM)
                    .width(iced::Length::Fill)
                    .shaping(iced::widget::text::Shaping::Advanced),
                iced::widget::text(if active { &entry.action } else { "" }).size(1. * crate::REM)
            ]
            .padding(0.5 * crate::REM),
        )
        .style(style(active)),
    )
    // We're fixing the height here to unify it
    // with the height of plugin headers for a smooth
    // scrolling experience
    .height(crate::ENTRY_HEIGHT)
    .padding(iced::Padding::from([0., 0.75 * crate::REM]))
    .into();
}

fn clipped_title(title: String) -> String {
    if title.char_indices().count() <= 57 {
        return title;
    }

    let mut clipped_title: String = title
        .char_indices()
        .map(|(_, character)| character)
        .take(57)
        .collect();
    clipped_title.push_str("...");
    clipped_title
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
        let color_settings = crate::settings::Settings::get_or_init();

        iced::widget::container::Appearance {
            background: None,
            border: iced::Border {
                color: crate::settings::hexcolor(&color_settings.color.text),
                width: 1.0,
                radius: iced::border::Radius::from(0.1 * crate::REM),
            },
            text_color: None,
            shadow: iced::Shadow::default(),
        }
    }
}
