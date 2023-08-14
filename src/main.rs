use iced::widget::{button, column, text, text_input};
use iced::window::PlatformSpecific;
use iced::{window, Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    let mut default_settings = Settings::default();
    default_settings.window = window::Settings {
        transparent: true,
        size: (500, 500),
        decorations: false,
        always_on_top: true,
        resizable: false,
        position: window::Position::Centered,
        min_size: None,
        max_size: None,
        icon: None,
        visible: true,
        platform_specific: PlatformSpecific::default(),
    };

    Centerpiece::run(default_settings)
}

struct Centerpiece {
    value: i32,
    query: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Centerpiece {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            query: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Centerpiece")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::InputChanged(input) => {
                self.query = input;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Search", &self.query)
                .on_input(Message::InputChanged)
                .padding(32),
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
