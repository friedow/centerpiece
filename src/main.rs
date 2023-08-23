use iced::Application;

mod component;
mod model;
mod style;

pub fn main() -> iced::Result {
    let mut settings = iced::Settings::default();
    settings.default_text_size = style::REM;
    settings.default_font = iced::Font {
        family: iced::font::Family::Name("FiraCode Nerd Font"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        monospaced: true,
    };

    settings.window = iced::window::Settings {
        transparent: true,
        size: (550, 350),
        decorations: false,
        level: iced::window::Level::AlwaysOnTop,
        resizable: false,
        position: iced::window::Position::Centered,
        min_size: None,
        max_size: None,
        icon: None,
        visible: true,
        platform_specific: iced::window::PlatformSpecific::default(),
    };

    Centerpiece::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    Search(String),
    Event(iced::Event),
    FontLoaded(Result<(), iced::font::Error>),
}

struct Centerpiece {
    query: String,
    active_entry_index: usize,
    plugins: Vec<model::Plugin>,
}

impl Application for Centerpiece {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        return (
            Self {
                query: String::from(""),
                active_entry_index: 0,
                plugins: vec![
                    model::Plugin {
                        id: String::from("clock"),
                        title: String::from("󰅐 Clock"),
                        entries: vec![
                            model::Entry {
                                id: String::from("clock-item-1"),
                                title: String::from("Item 1"),
                                action: String::from("open"),
                            },
                            model::Entry {
                                id: String::from("clock-item-2"),
                                title: String::from("Item 2"),
                                action: String::from("open"),
                            },
                        ],
                    },
                    model::Plugin {
                        id: String::from("git-repositories"),
                        title: String::from("Plugin 2"),
                        entries: vec![
                            model::Entry {
                                id: String::from("git-repo-item-1"),
                                title: String::from("Item 1"),
                                action: String::from("switch"),
                            },
                            model::Entry {
                                id: String::from("git-repo-item-2"),
                                title: String::from("Item 2"),
                                action: String::from("switch"),
                            },
                        ],
                    },
                ],
            },
            iced::Command::batch(vec![
                iced::font::load(
                    include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Regular.ttf").as_slice(),
                )
                .map(Message::FontLoaded),
                iced::Command::perform(async {}, move |()| Message::Loaded),
            ]),
        );
    }

    fn title(&self) -> String {
        String::from("Centerpiece")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Loaded => self.focus_search_input(),

            Message::Search(input) => {
                self.query = input;
                return iced::Command::none();
            }

            Message::Event(event) => match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Up,
                    ..
                }) => {
                    let entries = self.entries();
                    if entries.len() == 0 {
                        self.active_entry_index = 0;
                        return iced::Command::none();
                    }

                    if self.active_entry_index == 0 {
                        self.active_entry_index = entries.len() - 1;
                        return iced::Command::none();
                    }

                    self.active_entry_index -= 1;
                    return iced::Command::none();
                }

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Down,
                    ..
                }) => {
                    let entries = self.entries();
                    if entries.len() == 0 || self.active_entry_index == entries.len() - 1 {
                        self.active_entry_index = 0;
                        return iced::Command::none();
                    }

                    self.active_entry_index += 1;
                    return iced::Command::none();
                }

                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => self.focus_search_input(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                    key_code: iced::keyboard::KeyCode::Escape,
                    ..
                }) => iced::window::close(),

                _ => iced::Command::none(),
            },

            Message::FontLoaded(_) => iced::Command::none(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        return iced::subscription::events().map(Message::Event);
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::container(iced::widget::column![
            component::query_input::view(&self.query),
            iced::widget::column(
                self.plugins
                    .iter()
                    .map(|plugin| component::plugin::view(plugin, self.active_entry_id()))
                    .collect()
            ),
        ])
        .style(iced::theme::Container::Custom(Box::new(
            style::ApplicationWrapper {},
        )))
        .into()
    }

    fn theme(&self) -> iced::Theme {
        return iced::Theme::Dark;
    }

    fn style(&self) -> iced::theme::Application {
        return iced::theme::Application::Custom(Box::new(style::Sandbox {}));
    }
}

impl Centerpiece {
    fn entries(&self) -> Vec<&model::Entry> {
        return self
            .plugins
            .iter()
            .flat_map(|plugin| &plugin.entries)
            .collect();
    }

    fn active_entry_id(&self) -> Option<&String> {
        let entries = self.entries();
        let active_entry = entries.get(self.active_entry_index);
        return match active_entry {
            Some(entry) => Some(&entry.id),
            None => None,
        };
    }

    fn focus_search_input(&self) -> iced::Command<Message> {
        return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
            component::query_input::SEARCH_INPUT_ID,
        ));
    }
}
