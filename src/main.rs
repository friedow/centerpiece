use iced::Sandbox;

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
    InputChanged(String),
    IncrementPressed,
    DecrementPressed,
}

struct Centerpiece {
    value: i32,
    query: String,
    plugins: Vec<Plugin>,
}

impl Sandbox for Centerpiece {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            query: String::from(""),
            plugins: vec![
                Plugin {
                    title: String::from("Plugin 1"),
                    entries: vec![
                        PluginEntry {
                            title: String::from("Item 1"),
                            action: String::from("open"),
                        },
                        PluginEntry {
                            title: String::from("Item 2"),
                            action: String::from("open"),
                        },
                    ],
                },
                Plugin {
                    title: String::from("Plugin 2"),
                    entries: vec![
                        PluginEntry {
                            title: String::from("Item 1"),
                            action: String::from("switch"),
                        },
                        PluginEntry {
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
            Message::InputChanged(input) => {
                self.query = input;
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::column![
            iced::widget::text_input("Search", &self.query)
                .on_input(Message::InputChanged)
                .padding(5),
            iced::widget::column(self.plugins.iter().map(plugin).collect()),
        ]
        .padding(20)
        .into()
    }
}

struct Plugin {
    title: String,
    entries: Vec<PluginEntry>,
}

struct PluginEntry {
    title: String,
    action: String,
}

fn plugin(plugin: &Plugin) -> iced::Element<'static, Message> {
    return iced::widget::column![
        plugin_title(&plugin.title),
        iced::widget::column(plugin.entries.iter().map(entry).collect()),
    ]
    .into();
}

fn plugin_title(title: &String) -> iced::Element<'static, Message> {
    return iced::widget::row![iced::widget::text(title),].into();
}

fn entry(entry: &PluginEntry) -> iced::Element<'static, Message> {
    return iced::widget::row![
        iced::widget::text(&entry.title),
        iced::widget::text(&entry.action),
    ]
    .into();
}
