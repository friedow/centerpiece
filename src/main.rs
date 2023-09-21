use iced::Application;

mod component;
mod model;
mod plugin;

pub fn main() -> iced::Result {
    return Centerpiece::run(Centerpiece::settings());
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    Search(String),
    Event(iced::Event),
    FontLoaded(Result<(), iced::font::Error>),
    RegisterPlugin(model::Plugin),
    AppendEntry(String, model::Entry),
    Clear(String),
    Exit,
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
        let _ = iced::font::load(
            include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Regular.ttf").as_slice(),
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

            Message::AppendEntry(plugin_id, entry) => self.append_entry(plugin_id, entry),

            Message::Clear(plugin_id) => self.clear_entries(plugin_id),

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
            crate::plugin::applications::ApplicationsPlugin::spawn(),
            crate::plugin::clock::ClockPlugin::spawn(),
        ]);
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::container(iced::widget::column![
            component::query_input::view(&self.query),
            iced::widget::column(
                self.plugins
                    .iter()
                    .filter(|plugin| !plugin.entries.is_empty())
                    .map(|plugin| component::plugin::view(plugin, self.active_entry_id()))
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
    fn settings() -> iced::Settings<()> {
        let mut settings = iced::Settings::default();
        settings.default_text_size = REM;
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
        return settings;
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
        return iced::Command::none();
    }

    fn focus_search_input(&self) -> iced::Command<Message> {
        return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
            component::query_input::SEARCH_INPUT_ID,
        ));
    }

    fn select_previous_entry(&mut self) -> iced::Command<Message> {
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

    fn select_next_entry(&mut self) -> iced::Command<Message> {
        let entries = self.entries();
        if entries.len() == 0 || self.active_entry_index == entries.len() - 1 {
            self.active_entry_index = 0;
            return iced::Command::none();
        }

        self.active_entry_index += 1;
        return iced::Command::none();
    }

    fn register_plugin(&mut self, plugin: crate::model::Plugin) -> iced::Command<Message> {
        self.plugins.push(plugin);
        return iced::Command::none();
    }

    fn append_entry(
        &mut self,
        plugin_id: String,
        entry: crate::model::Entry,
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
        plugin.entries.push(entry);
        return iced::Command::none();
    }

    fn clear_entries(&mut self, plugin_id: String) -> iced::Command<Message> {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id);
        if plugin.is_none() {
            println!(
                "Clearing entries failed. Could not find plugin with id {:?}",
                plugin_id
            );
            return iced::Command::none();
        }

        let plugin = plugin.unwrap();
        plugin.entries.clear();
        return iced::Command::none();
    }

    fn activate_selected_entry(&mut self) -> iced::Command<Message> {
        println!("activating 1");
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

        println!("activating 2");

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
