use iced::Sandbox;

mod types;

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
    plugins: Vec<types::Plugin>,
}

const REM: f32 = 14.0;

impl Sandbox for Centerpiece {
    type Message = Message;

    fn new() -> Self {
        Self {
            query: String::from(""),
            active_entry_id: String::from("clock-item-1"),
            plugins: vec![
                types::Plugin {
                    id: String::from("clock"),
                    title: String::from("Plugin 1"),
                    entries: vec![
                        types::Entry {
                            id: String::from("clock-item-1"),
                            title: String::from("Item 1"),
                            action: String::from("open"),
                        },
                        types::Entry {
                            id: String::from("clock-item-2"),
                            title: String::from("Item 2"),
                            action: String::from("open"),
                        },
                    ],
                },
                types::Plugin {
                    id: String::from("git-repositories"),
                    title: String::from("Plugin 2"),
                    entries: vec![
                        types::Entry {
                            id: String::from("git-repo-item-1"),
                            title: String::from("Item 1"),
                            action: String::from("switch"),
                        },
                        types::Entry {
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
        iced::widget::column![
            iced::widget::text_input("Search", &self.query)
                .on_input(Message::Search)
                .size(1.0 * REM)
                .padding(iced::Padding::from([0.8 * REM, 1.0 * REM])),
            iced::widget::column(
                self.plugins
                    .iter()
                    .map(|plugin| view_plugin(plugin, &self.active_entry_id))
                    .collect()
            ),
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        return iced::Theme::Dark;
    }
}

fn view_plugin(
    plugin: &types::Plugin,
    active_entry_id: &String,
) -> iced::Element<'static, Message> {
    return iced::widget::column![
        iced::widget::horizontal_rule(1),
        iced::widget::column![
            view_plugin_title(&plugin.title),
            iced::widget::column(
                plugin
                    .entries
                    .iter()
                    .map(|entry| view_entry(entry, active_entry_id))
                    .collect()
            ),
        ]
        .padding(0.5 * REM),
    ]
    .into();
}

fn view_plugin_title(title: &String) -> iced::Element<'static, Message> {
    return iced::widget::row![iced::widget::text(title).size(0.75 * REM)]
        .padding(0.5 * REM)
        .into();
}

fn view_entry(entry: &types::Entry, active_entry_id: &String) -> iced::Element<'static, Message> {
    return iced::widget::container(
        iced::widget::row![
            iced::widget::text(&entry.title)
                .size(1.0 * REM)
                .width(iced::Length::Fill),
            iced::widget::text(&entry.action).size(1.0 * REM),
        ]
        .padding(0.5 * REM),
    )
    .style(if &entry.id == active_entry_id {
        iced::theme::Container::Box
    } else {
        iced::theme::Container::Transparent
    })
    .into();

    // iced::theme::Container::from(|| iced::widget::container::Appearance {
    //     text_color: None,
    //     background: None,
    //     border_radius: 5.0,
    //     border_width: 1.0,
    //     border_color: iced::Color::from_rgb(150, 150, 150),
    // })
}

// iced::widget::container::Appearance {
//     text_color: None,
//     background: None,
//     border_radius: 5.0,
//     border_width: 1.0,
//     border_color: iced::Color::from_rgb(150, 150, 150),
// }
