use iced::Application;

mod component;
mod model;
mod plugin;

pub fn main() -> iced::Result {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    return Centerpiece::run(Centerpiece::settings());
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

        return (
            Self {
                query: String::from(""),
                active_entry_index: 0,
                plugins: vec![],
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

            Message::Search(input) => self.search(input),

            Message::Event(event) => match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Up,
                    ..
                }) => self.select_previous_entry(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Down,
                    ..
                }) => self.select_next_entry(),

                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Enter,
                    ..
                }) => self.activate_selected_entry(),

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
        return iced::subscription::Subscription::batch(vec![
            iced::subscription::events_with(|event, _status| match event {
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
            }),
            crate::plugin::git_repositories::GitRepositoriesPlugin::spawn(),
            crate::plugin::windows::WindowsPlugin::spawn(),
            crate::plugin::applications::ApplicationsPlugin::spawn(),
            crate::plugin::resource_monitor::ResourceMonitorPlugin::spawn(),
            crate::plugin::clock::ClockPlugin::spawn(),
        ]);
    }

    fn view(&self) -> iced::Element<Message> {
        let entries = self.entries();

        iced::widget::container(iced::widget::column![
            component::query_input::view(&self.query, entries.len() > 0),
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
        return iced::Theme::Dark;
    }

    fn style(&self) -> iced::theme::Application {
        return iced::theme::Application::Custom(Box::new(SandboxStyle {}));
    }
}

impl Centerpiece {
    fn settings() -> iced::Settings<()> {
        let mut settings = iced::Settings::default();
        settings.default_text_size = REM;
        settings.default_font = iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            monospaced: true,
        };
        settings.id = Some(APP_ID.into());

        settings.window = iced::window::Settings {
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
        return settings;
    }

    fn platform_specific_settings() -> iced::window::PlatformSpecific {
        let mut specific = iced::window::PlatformSpecific::default();
        specific.application_id = APP_ID.into();
        specific
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
        return match active_entry {
            Some(entry) => Some(&entry.id),
            None => None,
        };
    }

    fn search(&mut self, input: String) -> iced::Command<Message> {
        for plugin in self.plugins.iter_mut() {
            let _ = plugin
                .app_channel_out
                .try_send(crate::model::PluginRequest::Search(input.clone()));
        }

        self.query = input;
        return self.select_first_entry();
    }

    fn focus_search_input(&self) -> iced::Command<Message> {
        return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
            component::query_input::SEARCH_INPUT_ID,
        ));
    }

    fn select_first_entry(&mut self) -> iced::Command<Message> {
        self.active_entry_index = 0;
        return self.scroll_to_selected_entry();
    }

    fn select_previous_entry(&mut self) -> iced::Command<Message> {
        let entries = self.entries();
        if entries.len() == 0 {
            return self.select_first_entry();
        }

        if self.active_entry_index == 0 {
            self.active_entry_index = entries.len() - 1;
            return self.scroll_to_selected_entry();
        }

        self.active_entry_index -= 1;
        return self.scroll_to_selected_entry();
    }

    fn select_next_entry(&mut self) -> iced::Command<Message> {
        let entries = self.entries();
        if entries.len() == 0 || self.active_entry_index == entries.len() - 1 {
            return self.select_first_entry();
        }

        self.active_entry_index += 1;
        return self.scroll_to_selected_entry();
    }

    fn scroll_to_selected_entry(&self) -> iced::Command<Message> {
        let total_entries = self.entries().len() as f32;
        let offset = (1.0 / (total_entries - 1.0)) * self.active_entry_index as f32;
        return iced::widget::scrollable::snap_to(
            iced::widget::scrollable::Id::new(SCROLLABLE_ID),
            iced::widget::scrollable::RelativeOffset { x: 0.0, y: offset },
        );
    }

    fn register_plugin(&mut self, plugin: crate::model::Plugin) -> iced::Command<Message> {
        self.plugins.push(plugin);
        self.plugins.sort_by(|a, b| b.priority.cmp(&a.priority));
        return iced::Command::none();
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
        return iced::Command::none();
    }

    fn activate_selected_entry(&mut self) -> iced::Command<Message> {
        let active_entry_id_option = self.active_entry_id();
        if active_entry_id_option.is_none() {
            return iced::Command::none();
        }
        let active_entry_id = active_entry_id_option.unwrap().clone();

        let plugin_option = self.plugins.iter_mut().find(|plugin| {
            plugin
                .entries
                .iter()
                .any(|entry| entry.id.eq(&active_entry_id))
        });
        if plugin_option.is_none() {
            return iced::Command::none();
        }
        let plugin = plugin_option.unwrap();

        plugin
            .app_channel_out
            .try_send(model::PluginRequest::Activate(active_entry_id.clone()))
            .ok();
        return iced::Command::none();
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
        return iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::Color::BLACK)),
            border_color: iced::Color::TRANSPARENT,
            border_radius: iced::BorderRadius::from(0.25 * REM),
            border_width: 0.,
            text_color: None,
        };
    }
}

struct ScrollableStyle {}
impl iced::widget::scrollable::StyleSheet for ScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Scrollbar {
        return iced::widget::scrollable::Scrollbar {
            background: None,
            border_radius: iced::BorderRadius::from(0.),
            border_width: 0.,
            border_color: iced::Color::TRANSPARENT,
            scroller: iced::widget::scrollable::Scroller {
                color: iced::Color::WHITE,
                border_radius: iced::BorderRadius::from(0.25 * REM),
                border_width: 4.,
                border_color: iced::Color::BLACK,
            },
        };
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Scrollbar {
        return iced::widget::scrollable::Scrollbar {
            background: None,
            border_radius: iced::BorderRadius::from(0.),
            border_width: 0.,
            border_color: iced::Color::TRANSPARENT,
            scroller: iced::widget::scrollable::Scroller {
                color: iced::Color::WHITE,
                border_radius: iced::BorderRadius::from(0.25 * REM),
                border_width: 4.,
                border_color: iced::Color::BLACK,
            },
        };
    }
}
