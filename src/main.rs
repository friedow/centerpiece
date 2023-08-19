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
        iced::widget::container(iced::widget::column![
            iced::widget::text_input("Search", &self.query)
                .on_input(Message::Search)
                .size(1.0 * REM)
                .padding(iced::Padding::from([0.8 * REM, 1. * REM]))
                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {}))),
            iced::widget::column(
                self.plugins
                    .iter()
                    .map(|plugin| self.view_plugin(plugin))
                    .collect()
            ),
        ])
        .style(iced::theme::Container::Custom(Box::new(
            ApplicationWrapperStyle {},
        )))
        .into()
    }

    fn theme(&self) -> iced::Theme {
        return iced::Theme::Dark;
    }

    fn style(&self) -> iced::theme::Application {
        return iced::theme::Application::Custom(Box::new(SandboxStyle {}));
    }
}

impl Centerpiece {
    fn view_plugin(&self, plugin: &types::Plugin) -> iced::Element<'static, Message> {
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
            .padding(0.5 * REM),
        ]
        .into();
    }

    fn view_plugin_title(&self, title: &String) -> iced::Element<'static, Message> {
        return iced::widget::row![iced::widget::text(title).size(0.75 * REM)]
            .padding(0.5 * REM)
            .into();
    }

    fn view_entry(&self, entry: &types::Entry) -> iced::Element<'static, Message> {
        return iced::widget::container(
            iced::widget::row![
                iced::widget::text(&entry.title)
                    .size(1. * REM)
                    .width(iced::Length::Fill),
                iced::widget::text(&entry.action).size(1. * REM),
            ]
            .padding(0.5 * REM),
        )
        .style(if entry.id == self.active_entry_id {
            iced::theme::Container::Custom(Box::new(ActiveEntryStyle {}))
        } else {
            iced::theme::Container::Transparent
        })
        .into();
    }
}

struct SandboxStyle {}
impl iced::application::StyleSheet for SandboxStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::color!(0x000000, 0.),
            text_color: iced::color!(0xffffff, 1.),
        }
    }
}

struct ApplicationWrapperStyle {}
impl iced::widget::container::StyleSheet for ApplicationWrapperStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::color!(0x000000, 1.))),
            border_color: iced::color!(0x000000, 0.),
            border_radius: 0.25 * REM,
            border_width: 0.,
            text_color: None,
        };
    }
}

struct TextInputStyle {}
impl iced::widget::text_input::StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        return iced::widget::text_input::Appearance {
            background: iced::Background::Color(iced::color!(0x000000, 0.)),
            border_radius: 0.,
            border_width: 0.,
            border_color: iced::color!(0x000000, 0.),
            icon_color: iced::color!(0xf3f3f3, 1.),
        };
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        return self.active(style);
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        return self.active(style);
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xf3f3f3, 1.);
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xffffff, 1.);
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0xfafafa, 1.);
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        return iced::color!(0x1b1b1b, 1.);
    }
}

struct ActiveEntryStyle {}
impl iced::widget::container::StyleSheet for ActiveEntryStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        return iced::widget::container::Appearance {
            background: None,
            border_radius: 0.1 * REM,
            border_width: 1.,
            border_color: iced::color!(0xffffff, 1.),
            text_color: None,
        };
    }
}
