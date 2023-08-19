use crate::component;
use crate::model;
use crate::style;
use crate::Message;

pub fn view(
    plugin: &model::Plugin,
    active_entry_id: Option<&String>,
) -> iced::Element<'static, Message> {
    return iced::widget::column![
        iced::widget::horizontal_rule(1),
        iced::widget::column![
            iced::widget::row![iced::widget::text(&plugin.title).size(0.75 * style::REM)]
                .padding(0.5 * style::REM),
            iced::widget::column(
                plugin
                    .entries
                    .iter()
                    .map(|entry| {
                        let is_active =
                            active_entry_id.is_some() && active_entry_id.unwrap() == &entry.id;
                        return component::entry::view(entry, is_active);
                    })
                    .collect()
            )
        ]
        .padding(0.5 * style::REM),
    ]
    .into();
}
