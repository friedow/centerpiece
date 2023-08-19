use crate::style;
use crate::Message;

pub const SEARCH_INPUT_ID: &str = "search_input";

pub fn view(query: &String) -> iced::Element<'static, Message> {
    return iced::widget::text_input("Search", query)
        .id(iced::widget::text_input::Id::new(SEARCH_INPUT_ID))
        .on_input(Message::Search)
        .size(1.0 * style::REM)
        .padding(iced::Padding::from([0.8 * style::REM, 1. * style::REM]))
        .style(iced::theme::TextInput::Custom(Box::new(
            style::TextInput {},
        )))
        .into();
}
