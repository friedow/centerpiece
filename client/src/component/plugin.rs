pub fn view(
    plugin: &crate::model::Plugin,
    add_horizontal_rule: bool,
    active_entry_id: Option<&String>,
) -> iced::Element<'static, crate::Message> {
    let mut view = iced::widget::column![];

    if add_horizontal_rule {
        view = view.push(iced::widget::horizontal_rule(1));
    }

    view = view.push(
        iced::widget::column![
            iced::widget::row![iced::widget::text(&plugin.title)
                .font(iced::Font {
                    family: iced::font::Family::Name("FiraCode Nerd Font"),
                    weight: iced::font::Weight::Light,
                    stretch: iced::font::Stretch::Normal,
                    style: iced::font::Style::default(),
                })
                .size(0.75 * crate::REM)]
            .padding(0.5 * crate::REM),
            iced::widget::column(
                plugin
                    .entries
                    .iter()
                    .map(|entry| {
                        let is_active =
                            active_entry_id.is_some() && active_entry_id.unwrap() == &entry.id;
                        crate::component::entry::view(entry, is_active)
                    })
                    .collect::<Vec<_>>()
            )
        ]
        .padding(0.75 * crate::REM),
    );

    view.into()
}
