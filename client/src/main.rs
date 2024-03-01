use clap::Parser;
use iced::Application;

mod cli;
mod component;
mod model;
mod plugin;
mod settings;

pub fn main() -> iced::Result {
    let _args = crate::cli::CliArgs::parse();
    simple_logger::init_with_level(log::Level::Info).unwrap();
    Centerpiece::run(Centerpiece::settings())
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    Search(String),
    Event(iced::Event),
    FontLoaded(Result<(), iced::font::Error>),
    RegisterPlugin(model::Plugin),
    UpdateEntries(String, Vec<model::Entry>),
    Exit,
}

struct Centerpiece {
    query: String,
    active_entry_index: usize,
    plugins: Vec<model::Plugin>,
}

pub const SCROLLABLE_ID: &str = "scrollable";
pub const APP_ID: &str = "centerpiece";

impl Application for Centerpiece {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let _ = iced::font::load(
            include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Regular.ttf").as_slice(),
        );
        let _ = iced::font::load(
            include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Light.ttf").as_slice(),
        );

        (
            Self {
                query: String::from(""),
                active_entry_index: 0,
                plugins: vec![],
            },
            iced::Command::perform(async {}, move |()| Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Centerpiece")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Loaded => self.focus_search_input(),

            Message::Search(input) => self.search(input),

            Message::Event(event) => match event {
                iced::Event::Keyboard(event) => match event {
                    iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                        if let iced::keyboard::Modifiers::CTRL = modifiers {
                            return match key.as_ref() {
                                iced::keyboard::Key::Character("j") => self.select_next_entry(),
                                iced::keyboard::Key::Character("k") => self.select_previous_entry(),
                                iced::keyboard::Key::Character("n") => self.select_next_plugin(),
                                iced::keyboard::Key::Character("p") => {
                                    self.select_previous_plugin()
                                }
                                _ => iced::Command::none(),
                            };
                        }
                        match key.as_ref() {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                                self.select_previous_entry()
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                                self.select_next_entry()
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => self
                                .activate_selected_entry()
                                .unwrap_or(iced::Command::none()),
                            _ => iced::Command::none(),
                        }
                    }
                    iced::keyboard::Event::KeyReleased { key, .. } => {
                        if key == iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) {
                            return iced::window::close(iced::window::Id::MAIN);
                        }
                        iced::Command::none()
                    }

                    _ => iced::Command::none(),
                },
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => self.focus_search_input(),
                _ => iced::Command::none(),
            },

            Message::FontLoaded(_) => iced::Command::none(),

            Message::RegisterPlugin(plugin) => self.register_plugin(plugin),

            Message::UpdateEntries(plugin_id, entries) => self.update_entries(plugin_id, entries),

            Message::Exit => iced::window::close(iced::window::Id::MAIN),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::Subscription::batch(vec![
            iced::event::listen_with(|event, _status| match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { .. }) => {
                    Some(Message::Event(event))
                }
                iced::Event::Keyboard(iced::keyboard::Event::KeyReleased { .. }) => {
                    Some(Message::Event(event))
                }
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => {
                    Some(Message::Event(event))
                }
                _ => None,
            }),
            crate::plugin::utils::spawn::<crate::plugin::windows::WindowsPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::applications::ApplicationsPlugin>(),
            crate::plugin::utils::spawn::<
                crate::plugin::brave::progressive_web_apps::ProgressiveWebAppsPlugin,
            >(),
            crate::plugin::utils::spawn::<crate::plugin::git_repositories::GitRepositoriesPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::brave::bookmarks::BookmarksPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::system::SystemPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::wifi::WifiPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::resource_monitor::battery::BatteryPlugin>(
            ),
            crate::plugin::utils::spawn::<crate::plugin::resource_monitor::cpu::CpuPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::resource_monitor::memory::MemoryPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::resource_monitor::disks::DisksPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::clock::ClockPlugin>(),
            crate::plugin::utils::spawn::<crate::plugin::brave::history::HistoryPlugin>(),
        ])
    }

    fn view(&self) -> iced::Element<Message> {
        let entries = self.entries();

        iced::widget::container(iced::widget::column![
            component::query_input::view(&self.query, !entries.is_empty()),
            iced::widget::scrollable(iced::widget::column(
                self.plugins
                    .iter()
                    .filter(|plugin| !plugin.entries.is_empty())
                    .enumerate()
                    .map(|(index, plugin)| component::plugin::view(
                        plugin,
                        index != 0,
                        self.active_entry_id()
                    ))
                    .collect::<Vec<_>>()
            ))
            .id(iced::widget::scrollable::Id::new(SCROLLABLE_ID))
            .style(iced::theme::Scrollable::Custom(Box::new(
                ScrollableStyle {},
            ))),
        ])
        .style(iced::theme::Container::Custom(Box::new(
            ApplicationWrapperStyle {},
        )))
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(Box::new(SandboxStyle {}))
    }
}

impl Centerpiece {
    fn settings() -> iced::Settings<()> {
        let default_text_size = iced::Pixels(REM);

        let default_font = iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::default(),
        };

        let id = Some(APP_ID.into());

        let window = iced::window::Settings {
            transparent: true,
            size: iced::Size::new(650.0, 400.0),
            decorations: false,
            level: iced::window::Level::AlwaysOnTop,
            resizable: false,
            position: iced::window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
            visible: true,
            platform_specific: Self::platform_specific_settings(),
            exit_on_close_request: true,
        };

        iced::Settings {
            id,
            window,
            default_font,
            default_text_size,
            ..Default::default()
        }
    }

    fn platform_specific_settings() -> iced::window::settings::PlatformSpecific {
        iced::window::settings::PlatformSpecific {
            application_id: APP_ID.into(),
        }
    }

    fn entries(&self) -> Vec<&model::Entry> {
        self.plugins
            .iter()
            .flat_map(|plugin| &plugin.entries)
            .collect()
    }

    fn active_entry_id(&self) -> Option<&String> {
        let entries = self.entries();
        let active_entry = entries.get(self.active_entry_index);
        match active_entry {
            Some(entry) => Some(&entry.id),
            None => None,
        }
    }

    fn search(&mut self, input: String) -> iced::Command<Message> {
        for plugin in self.plugins.iter_mut() {
            let _ = plugin
                .app_channel_out
                .try_send(crate::model::PluginRequest::Search(input.clone()));
        }

        self.query = input;
        self.select_first_entry()
    }

    fn focus_search_input(&self) -> iced::Command<Message> {
        iced::widget::text_input::focus(iced::widget::text_input::Id::new(
            component::query_input::SEARCH_INPUT_ID,
        ))
    }

    fn select_first_entry(&mut self) -> iced::Command<Message> {
        self.active_entry_index = 0;
        self.scroll_to_selected_entry()
    }

    fn select_previous_entry(&mut self) -> iced::Command<Message> {
        let entries = self.entries();
        if entries.is_empty() {
            return self.select_first_entry();
        }

        if self.active_entry_index == 0 {
            self.active_entry_index = entries.len() - 1;
            return self.scroll_to_selected_entry();
        }

        self.active_entry_index -= 1;
        self.scroll_to_selected_entry()
    }

    fn select_next_entry(&mut self) -> iced::Command<Message> {
        let entries = self.entries();
        if entries.is_empty() || self.active_entry_index == entries.len() - 1 {
            return self.select_first_entry();
        }

        self.active_entry_index += 1;
        self.scroll_to_selected_entry()
    }

    fn scroll_to_selected_entry(&self) -> iced::Command<Message> {
        let total_entries = self.entries().len() as f32;
        let offset = (1.0 / (total_entries - 1.0)) * self.active_entry_index as f32;
        iced::widget::scrollable::snap_to(
            iced::widget::scrollable::Id::new(SCROLLABLE_ID),
            iced::widget::scrollable::RelativeOffset { x: 0.0, y: offset },
        )
    }

    fn select_next_plugin(&mut self) -> iced::Command<Message> {
        let accumulated_entries = self
            .plugins
            .iter()
            .map(|plugin| plugin.entries.len())
            .scan(0, |acc, len| {
                let prev = *acc;
                *acc += len;
                Some(prev)
            })
            .find(|&total| total > self.active_entry_index)
            .unwrap_or(self.active_entry_index);

        self.active_entry_index = accumulated_entries;
        self.scroll_to_selected_entry()
    }

    fn select_previous_plugin(&mut self) -> iced::Command<Message> {
        if self.plugins.is_empty() || self.active_entry_index == 0 {
            return self.select_first_entry();
        }

        let accumulated_entries = self
            .plugins
            .iter()
            .map(|plugin| plugin.entries.len())
            .scan(0, |acc, len| {
                let prev = *acc;
                *acc += len;
                Some(prev)
            })
            .take_while(|&total| total < self.active_entry_index)
            .last()
            .unwrap_or(0);

        self.active_entry_index = accumulated_entries;
        self.scroll_to_selected_entry()
    }

    fn register_plugin(&mut self, plugin: crate::model::Plugin) -> iced::Command<Message> {
        self.plugins.push(plugin);
        self.plugins.sort_by(|a, b| b.priority.cmp(&a.priority));
        iced::Command::none()
    }

    fn update_entries(
        &mut self,
        plugin_id: String,
        entries: Vec<crate::model::Entry>,
    ) -> iced::Command<Message> {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id);
        if plugin.is_none() {
            println!(
                "Appending entry failed. Could not find plugin with id {:?}",
                plugin_id
            );
            return iced::Command::none();
        }

        let plugin = plugin.unwrap();
        plugin.entries = entries;
        iced::Command::none()
    }

    fn activate_selected_entry(&mut self) -> Option<iced::Command<Message>> {
        let active_entry_id = self.active_entry_id()?.clone();

        let entry = self
            .entries()
            .into_iter()
            .find(|entry| entry.id == *active_entry_id)?
            .clone();

        let plugin = self.plugins.iter_mut().find(|plugin| {
            plugin
                .entries
                .iter()
                .any(|entry| entry.id == *active_entry_id)
        })?;

        plugin
            .app_channel_out
            .try_send(model::PluginRequest::Activate(entry))
            .ok();
        Some(iced::Command::none())
    }
}

pub const REM: f32 = 14.0;

struct SandboxStyle {}
impl iced::application::StyleSheet for SandboxStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: iced::Color::WHITE,
        }
    }
}

struct ApplicationWrapperStyle {}
impl iced::widget::container::StyleSheet for ApplicationWrapperStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::Color::BLACK)),
            text_color: None,
            shadow: iced::Shadow::default(),
            border: iced::Border {
                radius: iced::border::Radius::from(0.25 * REM),
                width: 0.,
                color: iced::Color::TRANSPARENT,
            },
        }
    }
}

struct ScrollableStyle {}
impl iced::widget::scrollable::StyleSheet for ScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: None,
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color::WHITE,
                    border: iced::Border {
                        radius: iced::border::Radius::from(0.25 * REM),
                        width: 4.,
                        color: iced::Color::BLACK,
                    },
                },
                border: iced::Border {
                    radius: iced::border::Radius::from(0.),
                    width: 0.,
                    color: iced::Color::TRANSPARENT,
                },
            },
            container: iced::widget::container::Appearance::default(),
            gap: None,
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: None,
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color::WHITE,
                    border: iced::Border {
                        radius: iced::border::Radius::from(0.25 * REM),
                        width: 4.,
                        color: iced::Color::BLACK,
                    },
                },
                border: iced::Border {
                    radius: iced::border::Radius::from(0.),
                    width: 0.,
                    color: iced::Color::TRANSPARENT,
                },
            },
            container: iced::widget::container::Appearance::default(),
            gap: None,
        }
    }
}
