use iced::Sandbox;

mod model;
mod style;

pub fn main() -> iced::Result {
    let mut default_settings = iced::Settings::default();

    default_settings.window = iced::window::Settings {
        transparent: true,
        size: (500, 350),
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
    Search(String),
}

struct Centerpiece {
    query: String,
    active_entry_id: String,
    plugins: Vec<model::Plugin>,
}

impl Sandbox for Centerpiece {
    type Message = Message;

    fn new() -> Self {
        Self {
            query: String::from(""),
            active_entry_id: String::from("clock-item-1"),
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
        }
    }

    fn title(&self) -> String {
        String::from("Centerpiece")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Search(input) => {
                self.query = input;
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::container(iced::widget::column![
            iced::widget::text_input("Search", &self.query)
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
    fn view_plugin(&self, plugin: &model::Plugin) -> iced::Element<'static, Message> {
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

    fn view_plugin_title(&self, title: &String) -> iced::Element<'static, Message> {
        return iced::widget::row![iced::widget::text(title).size(0.75 * style::REM)]
            .padding(0.5 * style::REM)
            .into();
    }

    fn view_entry(&self, entry: &model::Entry) -> iced::Element<'static, Message> {
        return iced::widget::container(
            iced::widget::row![
                iced::widget::text(&entry.title)
                    .size(1. * style::REM)
                    .width(iced::Length::Fill),
                iced::widget::text(&entry.action).size(1. * style::REM),
            ]
            .padding(0.5 * style::REM),
        )
        .style(if entry.id == self.active_entry_id {
            iced::theme::Container::Custom(Box::new(style::ActiveEntry {}))
        } else {
            iced::theme::Container::Transparent
        })
        .into();
    }
}
