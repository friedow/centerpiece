pub fn view(entry: &crate::model::Entry, active: bool) -> iced::Element<'static, crate::Message> {
    iced::widget::container(
        iced::widget::container(
            iced::widget::row![
                iced::widget::text(clipped_title(entry.title.clone()))
                    .size(1. * crate::rem())
                    .width(iced::Length::Fill)
                    .shaping(iced::widget::text::Shaping::Advanced),
                iced::widget::text(if active {
                    entry.action.clone()
                } else {
                    "".to_string()
                })
                .size(1. * crate::rem())
            ]
            .padding(0.5 * crate::rem()),
        )
        .style(move |theme: &iced::Theme| {
            if !active {
                return iced::widget::container::Style::default();
            };

            let palette = theme.extended_palette();
            iced::widget::container::Style {
                border: iced::Border {
                    color: palette.background.base.text,
                    width: 1.,
                    radius: iced::border::Radius::from(0.1 * crate::rem()),
                },
                ..Default::default()
            }
        }),
    )
    // We're fixing the height here to unify it
    // with the height of plugin headers for a smooth
    // scrolling experience
    .height(crate::entry_height())
    .padding([0., 0.75 * crate::rem()])
    .into()
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
