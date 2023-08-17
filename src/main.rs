use iced::Sandbox;

mod types;

pub fn main() -> iced::Result {
    let mut default_settings = iced::Settings::default();

    default_settings.window = iced::window::Settings {
        transparent: true,
        size: (500, 400),
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
    IncrementPressed,
    DecrementPressed,
}

struct Centerpiece {
    value: i32,
    query: String,
    plugins: Vec<types::Plugin>,
}

impl Sandbox for Centerpiece {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            query: String::from(""),
            plugins: vec![
                types::Plugin {
                    title: String::from("Plugin 1"),
                    entries: vec![
                        types::PluginEntry {
                            title: String::from("Item 1"),
                            action: String::from("open"),
                        },
                        types::PluginEntry {
                            title: String::from("Item 2"),
                            action: String::from("open"),
                        },
                    ],
                },
                types::Plugin {
                    title: String::from("Plugin 2"),
                    entries: vec![
                        types::PluginEntry {
                            title: String::from("Item 1"),
                            action: String::from("switch"),
                        },
                        types::PluginEntry {
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
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Search(input) => {
                self.query = input;
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::column![
            iced::widget::text_input("Search", &self.query)
                .on_input(Message::Search)
                // TODO: define font size in rem
                .padding(16),
            iced::widget::column(self.plugins.iter().map(view_plugin).collect()),
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        return iced::Theme::Dark;
    }
}

fn view_plugin(plugin: &types::Plugin) -> iced::Element<'static, Message> {
    return iced::widget::column![
        iced::widget::horizontal_rule(1),
        iced::widget::column![
            view_plugin_title(&plugin.title),
            iced::widget::column(plugin.entries.iter().map(view_entry).collect()),
        ]
        .padding(8),
    ]
    .into();
}

fn view_plugin_title(title: &String) -> iced::Element<'static, Message> {
    return iced::widget::row![iced::widget::text(title).size(10)]
        .padding(8)
        .into();
}

fn view_entry(entry: &types::PluginEntry) -> iced::Element<'static, Message> {
    return iced::widget::row![
        iced::widget::text(&entry.title).width(iced::Length::Fill),
        iced::widget::text(&entry.action).horizontal_alignment(iced::alignment::Horizontal::Right),
    ]
    .padding(5)
    .into();
}
