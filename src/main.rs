use iced::Application;

mod model;
mod style;

pub fn main() -> iced::Result {
    let mut default_settings = iced::Settings::default();

    default_settings.window = iced::window::Settings {
        transparent: true,
        size: (450, 350),
        decorations: false,
        always_on_top: true,
        resizable: false,
        position: iced::window::Position::Centered,
        min_size: None,
        max_size: None,
        icon: None,
        visible: true,
        platform_specific: iced::window::PlatformSpecific::default(),
    };

    Centerpiece::run(default_settings)
}

#[derive(Debug, Clone)]
enum Message {
    Loaded,
    Search(String),
    Event(iced::Event),
}

const SEARCH_INPUT_ID: &str = "search_input";

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
                        title: String::from("Plugin 1"),
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
            iced::Command::perform(async {}, move |()| Message::Loaded),
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
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        return iced::subscription::events().map(Message::Event);
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::container(iced::widget::column![
            iced::widget::text_input("Search", &self.query)
                .id(iced::widget::text_input::Id::new(SEARCH_INPUT_ID))
                .on_input(Message::Search)
                .size(1.0 * style::REM)
                .padding(iced::Padding::from([0.8 * style::REM, 1. * style::REM]))
                .style(iced::theme::TextInput::Custom(Box::new(
                    style::TextInput {}
                ))),
            iced::widget::column(
                self.plugins
                    .iter()
                    .map(|plugin| self.view_plugin(plugin))
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
    fn view_plugin(&self, plugin: &model::Plugin) -> iced::Element<Message> {
        return iced::widget::column![
            iced::widget::horizontal_rule(1),
            iced::widget::column![
                self.view_plugin_title(&plugin.title),
                iced::widget::column(
                    plugin
                        .entries
                        .iter()
                        .map(|entry| self.view_entry(entry))
                        .collect()
                ),
            ]
            .padding(0.5 * style::REM),
        ]
        .into();
    }

    fn view_plugin_title(&self, title: &String) -> iced::Element<Message> {
        return iced::widget::row![iced::widget::text(title).size(0.75 * style::REM)]
            .padding(0.5 * style::REM)
            .into();
    }

    fn view_entry(&self, entry: &model::Entry) -> iced::Element<Message> {
        let entries = self.entries();
        let active_entry = entries.get(self.active_entry_index);

        return iced::widget::container(
            iced::widget::row![
                iced::widget::text(&entry.title)
                    .size(1. * style::REM)
                    .width(iced::Length::Fill),
                iced::widget::text(&entry.action).size(1. * style::REM),
            ]
            .padding(0.5 * style::REM),
        )
        .style(
            if active_entry.is_some() && active_entry.unwrap().id == entry.id {
                iced::theme::Container::Custom(Box::new(style::ActiveEntry {}))
            } else {
                iced::theme::Container::Transparent
            },
        )
        .into();
    }

    fn entries(&self) -> Vec<&model::Entry> {
        return self
            .plugins
            .iter()
            .flat_map(|plugin| &plugin.entries)
            .collect();
    }

    fn focus_search_input(&self) -> iced::Command<Message> {
        return iced::widget::text_input::focus(iced::widget::text_input::Id::new(SEARCH_INPUT_ID));
    }
}
