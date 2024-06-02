use clap::Parser;
use iced::Application;

mod cli;
mod component;
mod model;
mod plugin;
mod settings;

pub fn main() -> iced::Result {
    let args = crate::cli::CliArgs::parse();
    simple_logger::init_with_level(log::Level::Info).unwrap();
    Centerpiece::run(Centerpiece::settings(args))
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
    settings: settings::Settings,
}

pub const SCROLLABLE_ID: &str = "scrollable";
pub const APP_ID: &str = "centerpiece";

impl Application for Centerpiece {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = crate::cli::CliArgs;

    fn new(flags: crate::cli::CliArgs) -> (Self, iced::Command<Message>) {
        let settings = crate::settings::Settings::try_from(flags).unwrap_or_else(|_| {
            eprintln!("There is an issue with the settings, please check the configuration file.");
            std::process::exit(0);
        });

        (
            Self {
                query: String::from(""),
                active_entry_index: 0,
                plugins: vec![],
                settings,
            },
            iced::Command::batch(vec![
                iced::font::load(
                    include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Regular.ttf").as_slice(),
                )
                .map(Message::FontLoaded),
                iced::font::load(
                    include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Light.ttf").as_slice(),
                )
                .map(Message::FontLoaded),
                iced::Command::perform(async {}, move |()| Message::Loaded),
            ]),
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
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Up,
                    ..
                })
                | iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::K,
                    modifiers: iced::keyboard::Modifiers::CTRL,
                }) => self.select_previous_entry(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Down,
                    ..
                })
                | iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::J,
                    modifiers: iced::keyboard::Modifiers::CTRL,
                }) => self.select_next_entry(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::N,
                    modifiers: iced::keyboard::Modifiers::CTRL,
                }) => self.select_next_plugin(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::P,
                    modifiers: iced::keyboard::Modifiers::CTRL,
                }) => self.select_previous_plugin(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Enter,
                    ..
                }) => self
                    .activate_selected_entry()
                    .unwrap_or(iced::Command::none()),

                iced::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                    key_code: iced::keyboard::KeyCode::Escape,
                    ..
                }) => iced::window::close(),

                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => self.focus_search_input(),

                _ => iced::Command::none(),
            },

            Message::FontLoaded(_) => iced::Command::none(),

            Message::RegisterPlugin(plugin) => self.register_plugin(plugin),

            Message::UpdateEntries(plugin_id, entries) => self.update_entries(plugin_id, entries),

            Message::Exit => iced::window::close(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut subscriptions = vec![iced::subscription::events_with(
            |event, _status| match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    modifiers: _,
                    key_code: _,
                }) => Some(Message::Event(event)),
                iced::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                    modifiers: _,
                    key_code: _,
                }) => Some(Message::Event(event)),
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => {
                    Some(Message::Event(event))
                }
                _ => None,
            },
        )];

        if self.settings.plugin.applications.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::applications::ApplicationsPlugin,
            >());
        }

        if self.settings.plugin.brave_bookmarks.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::bookmarks::BookmarksPlugin,
            >());
        }

        if self.settings.plugin.brave_progressive_web_apps.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::progressive_web_apps::ProgressiveWebAppsPlugin,
            >());
        }

        if self.settings.plugin.brave_history.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::history::HistoryPlugin,
            >());
        }

        if self.settings.plugin.clock.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::clock::ClockPlugin,
            >());
        }

        if self.settings.plugin.firefox_bookmarks.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::firefox::bookmarks::BookmarksPlugin,
            >());
        }

        if self.settings.plugin.firefox_history.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::firefox::history::HistoryPlugin,
            >());
        }

        if self.settings.plugin.git_repositories.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::git_repositories::GitRepositoriesPlugin,
            >());
        }

        if self.settings.plugin.gitmoji.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::gitmoji::GitmojiPlugin,
            >());
        }

        if self.settings.plugin.resource_monitor_battery.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::battery::BatteryPlugin,
            >());
        }

        if self.settings.plugin.resource_monitor_cpu.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::cpu::CpuPlugin,
            >());
        }

        if self.settings.plugin.resource_monitor_disks.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::disks::DisksPlugin,
            >());
        }

        if self.settings.plugin.resource_monitor_memory.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::memory::MemoryPlugin,
            >());
        }

        if self.settings.plugin.system.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::system::SystemPlugin,
            >());
        }

        if self.settings.plugin.wifi.enable {
            subscriptions.push(crate::plugin::utils::spawn::<crate::plugin::wifi::WifiPlugin>());
        }

        if self.settings.plugin.sway_windows.enable {
            subscriptions.push(crate::plugin::utils::spawn::<
                crate::plugin::sway_windows::SwayWindowsPlugin,
            >());
        }

        iced::subscription::Subscription::batch(subscriptions)
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
                    .collect()
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
    fn settings(flags: crate::cli::CliArgs) -> iced::Settings<crate::cli::CliArgs> {
        let default_text_size = REM;

        let default_font = iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            monospaced: true,
        };

        let id = Some(APP_ID.into());

        let window = iced::window::Settings {
            transparent: true,
            size: (650, 400),
            decorations: false,
            level: iced::window::Level::AlwaysOnTop,
            resizable: false,
            position: iced::window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
            visible: true,
            platform_specific: Self::platform_specific_settings(),
        };

        iced::Settings {
            id,
            window,
            default_font,
            default_text_size,
            flags,
            ..Default::default()
        }
    }

    fn platform_specific_settings() -> iced::window::PlatformSpecific {
        iced::window::PlatformSpecific {
            application_id: APP_ID.into(),
        }
    }

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
        let plugin_index = match self.active_entry_id() {
            Some(active_entry_id) => self
                .plugins
                .iter()
                .filter(|plugin| plugin.entries.len() > 0)
                .position(|plugin| {
                    plugin
                        .entries
                        .iter()
                        .any(|entry| entry.id.eq(active_entry_id))
                })
                .unwrap_or(0) as f32,
            None => 0.0,
        };
        let entry_index = self.active_entry_index as f32;

        // 1.0 REM line height +
        // 2x0.5 REM padding +
        // 0.3 REM for good luck :D
        let entry_height = 2.3 * crate::REM;
        // 0.75 REM line height +
        // 2x0.5 REM padding +
        // 2x0.75 REM padding  +
        // 0.32 REM for good luck :D
        let plugin_header_height = 3.57 * crate::REM;

        let offset = (plugin_index * plugin_header_height) + (entry_index * entry_height);
        iced::widget::scrollable::scroll_to(
            iced::widget::scrollable::Id::new(SCROLLABLE_ID),
            iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: offset },
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

    fn register_plugin(&mut self, mut plugin: crate::model::Plugin) -> iced::Command<Message> {
        let _ = plugin
            .app_channel_out
            .try_send(crate::model::PluginRequest::Search(self.query.clone()));
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
        let color_settings = crate::settings::Settings::new();

        iced::application::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: settings::hexcolor(&color_settings.color.text),
        }
    }
}

struct ApplicationWrapperStyle {}
impl iced::widget::container::StyleSheet for ApplicationWrapperStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let color_settings = crate::settings::Settings::new();
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(settings::hexcolor(
                &color_settings.color.background,
            ))),
            border_color: iced::Color::TRANSPARENT,
            border_radius: iced::BorderRadius::from(0.25 * REM),
            border_width: 0.,
            text_color: None,
        }
    }
}

struct ScrollableStyle {}
impl iced::widget::scrollable::StyleSheet for ScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Scrollbar {
        let color_settings = crate::settings::Settings::new();
        iced::widget::scrollable::Scrollbar {
            background: None,
            border_radius: iced::BorderRadius::from(0.),
            border_width: 0.,
            border_color: iced::Color::TRANSPARENT,
            scroller: iced::widget::scrollable::Scroller {
                color: settings::hexcolor(&color_settings.color.surface),
                border_radius: iced::BorderRadius::from(0.25 * REM),
                border_width: 4.,
                border_color: settings::hexcolor(&color_settings.color.background),
            },
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Scrollbar {
        let color_settings = crate::settings::Settings::new();
        iced::widget::scrollable::Scrollbar {
            background: None,
            border_radius: iced::BorderRadius::from(0.),
            border_width: 0.,
            border_color: iced::Color::TRANSPARENT,
            scroller: iced::widget::scrollable::Scroller {
                color: settings::hexcolor(&color_settings.color.surface),
                border_radius: iced::BorderRadius::from(0.25 * REM),
                border_width: 4.,
                border_color: settings::hexcolor(&color_settings.color.background),
            },
        }
    }
}
