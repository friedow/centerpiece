pub fn view(
    plugin: &crate::model::Plugin,
    active_entry_id: Option<&String>,
) -> iced::Element<'static, crate::Message> {
    return iced::widget::column![
        iced::widget::horizontal_rule(1),
        iced::widget::column![
            iced::widget::row![iced::widget::text(&plugin.title).size(0.75 * crate::REM)]
                .padding(0.5 * crate::REM),
            iced::widget::column(
                plugin
                    .entries
                    .iter()
                    .map(|entry| {
                        let is_active =
                            active_entry_id.is_some() && active_entry_id.unwrap() == &entry.id;
                        return crate::component::entry::view(entry, is_active);
                    })
                    .collect()
            )
        ]
        .padding(0.5 * crate::REM),
    ]
    .into();
}
