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
    .style(if active {
        iced::theme::Container::Custom(Box::new(style::ActiveEntry {}))
    } else {
        iced::theme::Container::Transparent
    })
    .into();
}
