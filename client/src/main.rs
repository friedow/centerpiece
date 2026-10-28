use clap::Parser;
use egui::{self};

mod component;
mod lock;
mod model;
mod plugin;

const APP_ID: &str = "centerpiece";
use smithay_client_toolkit::shell::WaylandSurface;

pub fn main() {
    let args = settings::cli::CliArgs::parse();
    settings::Settings::try_from(args).unwrap_or_else(|_| {
        eprintln!("There is an issue with the settings, please check the configuration file.");
        std::process::exit(1);
    });

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let _lock = lock::LockFile::acquire();

    // Create channel for external events
    let (tx, rx) = std::sync::mpsc::channel::<AppEvent>();

    let mut app = wayapp::Application::new(move |t| {
        let _ = tx.send(AppEvent::WaylandDispatch(t));
    });

    let layer_surface = app.layer_shell.create_layer_surface(
        &app.qh,
        app.compositor_state.create_surface(&app.qh),
        smithay_client_toolkit::shell::wlr_layer::Layer::Overlay,
        Some(APP_ID),
        None,
    );
    layer_surface.set_keyboard_interactivity(
        smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::Exclusive,
    );
    layer_surface.set_margin(0, 0, 0, 0);
    layer_surface.set_size(800, 600);
    layer_surface.commit();

    let mut my_app = Centerpiece::new(&layer_surface);
    let mut egui_surface = wayapp::EguiSurfaceState::new(&app, &layer_surface, 800, 600);

    // Run the Wayland event loop
    app.run_dispatcher();

    loop {
        if let Ok(event) = rx.recv() {
            match event {
                AppEvent::WaylandDispatch(token) => {
                    let events = app.dispatch_pending(token);
                    egui_surface.handle_events(&mut app, &events, &mut |ui| my_app.update(ui));
                } // Handle other events here
            }
        }
    }
}

enum AppEvent {
    WaylandDispatch(wayapp::DispatchToken),
    // Other events can be added here
}

#[derive(Debug, Clone)]
pub enum Message {
    RegisterPlugin(model::Plugin),
    UpdateEntries(String, Vec<model::Entry>),
    Exit,
}

#[derive(Default)]
struct Centerpiece {
    query: String,
    active_plugin_id: Option<String>,
    active_entry_id: Option<String>,
    plugins: Vec<model::Plugin>,
    plugin_channels: Vec<async_channel::Receiver<Message>>,
}

impl Centerpiece {
    fn new(_cc: &smithay_client_toolkit::shell::wlr_layer::LayerSurface) -> Self {
        let mut centerpiece = Self::default();
        println!("creating centerpiece");
        centerpiece.launch_plugins();
        centerpiece
    }

    fn launch_plugins(self: &mut Centerpiece) {
        let settings = settings::Settings::get_or_init();

        if settings.plugin.applications.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::applications::ApplicationsPlugin,
            >());
        }

        if settings.plugin.brave_bookmarks.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::bookmarks::BookmarksPlugin,
            >());
        }

        if settings.plugin.brave_progressive_web_apps.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::progressive_web_apps::ProgressiveWebAppsPlugin,
            >());
        }

        if settings.plugin.brave_history.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::brave::history::HistoryPlugin,
            >());
        }

        if settings.plugin.clock.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::clock::ClockPlugin,
            >());
        }

        if settings.plugin.firefox_bookmarks.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::firefox::bookmarks::BookmarksPlugin,
            >());
        }

        if settings.plugin.firefox_history.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::firefox::history::HistoryPlugin,
            >());
        }

        if settings.plugin.git_repositories.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::git_repositories::GitRepositoriesPlugin,
            >());
        }

        if settings.plugin.gitmoji.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::gitmoji::GitmojiPlugin,
            >());
        }

        if settings.plugin.resource_monitor_battery.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::battery::BatteryPlugin,
            >());
        }

        if settings.plugin.resource_monitor_cpu.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::cpu::CpuPlugin,
            >());
        }

        if settings.plugin.resource_monitor_disks.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::disks::DisksPlugin,
            >());
        }

        if settings.plugin.resource_monitor_memory.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::resource_monitor::memory::MemoryPlugin,
            >());
        }

        if settings.plugin.system.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::system::SystemPlugin,
            >());
        }

        if settings.plugin.wifi.enable {
            self.plugin_channels
                .push(crate::plugin::utils::spawn::<crate::plugin::wifi::WifiPlugin>());
        }

        if settings.plugin.niri_windows.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::niri_windows::NiriWindowsPlugin,
            >());
        }

        if settings.plugin.sway_windows.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::sway_windows::SwayWindowsPlugin,
            >());
        }
    }

    fn exit() -> ! {
        lock::LockFile::cleanup();
        std::process::exit(0);
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.activate_selected_entry();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            Self::exit();
        }
        if ctx.input(|i| {
            (i.modifiers.ctrl && i.key_pressed(egui::Key::K)) || i.key_pressed(egui::Key::ArrowUp)
        }) {
            self.select_previous_entry();
        }
        if ctx.input(|i| {
            (i.modifiers.ctrl && i.key_pressed(egui::Key::J)) || i.key_pressed(egui::Key::ArrowDown)
        }) {
            self.select_next_entry();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::P)) {
            self.select_previous_plugin();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::N)) {
            self.select_next_plugin();
        }
    }

    fn handle_messages(&mut self) {
        let mut messages = vec![];

        for plugin_channel in &self.plugin_channels {
            let message_result = plugin_channel.try_recv();
            if message_result.is_err() {
                continue;
            }
            messages.push(message_result.unwrap());
        }

        for message in messages {
            match message {
                Message::RegisterPlugin(plugin) => self.register_plugin(plugin),

                Message::UpdateEntries(plugin_id, entries) => {
                    self.update_entries(plugin_id, entries)
                }

                Message::Exit => {
                    Self::exit();
                }
            }
        }
    }

    fn non_empty_plugins(&self) -> Vec<&model::Plugin> {
        self.plugins
            .iter()
            .filter(|plugin| !plugin.entries.is_empty())
            .collect()
    }

    /// Resolves the selection to (plugin index, entry index) within `plugins`.
    /// Ids that no longer resolve fall back to the first plugin or entry.
    fn active_position(&self, plugins: &[&model::Plugin]) -> Option<(usize, usize)> {
        let plugin_index = self
            .active_plugin_id
            .as_ref()
            .and_then(|id| plugins.iter().position(|plugin| &plugin.id == id))
            .unwrap_or(0);
        let plugin = plugins.get(plugin_index)?;
        let entry_index = self
            .active_entry_id
            .as_ref()
            .and_then(|id| plugin.entries.iter().position(|entry| &entry.id == id))
            .unwrap_or(0);
        Some((plugin_index, entry_index))
    }

    fn active_entry(&self) -> Option<(&model::Plugin, &model::Entry)> {
        let plugins = self.non_empty_plugins();
        let (plugin_index, entry_index) = self.active_position(&plugins)?;
        let plugin = plugins[plugin_index];
        Some((plugin, &plugin.entries[entry_index]))
    }

    /// Drops selection ids that no longer resolve, so the stored selection
    /// matches the entry rendered as active.
    fn normalize_selection(&mut self) {
        let plugins = self.non_empty_plugins();
        let plugin = self
            .active_plugin_id
            .as_ref()
            .and_then(|id| plugins.iter().find(|plugin| &plugin.id == id));
        let plugin_exists = plugin.is_some();
        let entry_exists = plugin.is_some_and(|plugin| {
            self.active_entry_id
                .as_ref()
                .is_some_and(|id| plugin.entries.iter().any(|entry| &entry.id == id))
        });

        if !plugin_exists {
            self.active_plugin_id = None;
        }
        if !entry_exists {
            self.active_entry_id = None;
        }
    }

    /// Moves the selection to the position returned by `next_position`,
    /// which receives the non-empty plugins and the current position.
    fn select_position(
        &mut self,
        next_position: impl FnOnce(&[&model::Plugin], usize, usize) -> (usize, usize),
    ) {
        let plugins = self.non_empty_plugins();
        let Some((plugin_index, entry_index)) = self.active_position(&plugins) else {
            return;
        };
        let (plugin_index, entry_index) = next_position(&plugins, plugin_index, entry_index);
        let plugin = plugins[plugin_index];
        let plugin_id = plugin.id.clone();
        let entry_id = plugin.entries[entry_index].id.clone();

        self.active_plugin_id = Some(plugin_id);
        self.active_entry_id = Some(entry_id);
    }

    fn search(&mut self) {
        for plugin in self.plugins.iter_mut() {
            let _ = plugin
                .app_channel_out
                .send_blocking(crate::model::PluginRequest::Search(self.query.clone()));
        }

        self.select_first_entry();
    }

    fn select_first_entry(&mut self) {
        self.active_plugin_id = None;
        self.active_entry_id = None;
    }

    fn select_previous_entry(&mut self) {
        self.select_position(|plugins, plugin_index, entry_index| {
            if entry_index > 0 {
                return (plugin_index, entry_index - 1);
            }
            let previous_plugin_index = (plugin_index + plugins.len() - 1) % plugins.len();
            (
                previous_plugin_index,
                plugins[previous_plugin_index].entries.len() - 1,
            )
        });
    }

    fn select_next_entry(&mut self) {
        self.select_position(|plugins, plugin_index, entry_index| {
            if entry_index + 1 < plugins[plugin_index].entries.len() {
                return (plugin_index, entry_index + 1);
            }
            ((plugin_index + 1) % plugins.len(), 0)
        });
    }

    /// Jumps to the first entry of the active plugin, or to the first entry
    /// of the previous plugin when already there.
    fn select_previous_plugin(&mut self) {
        self.select_position(|_, plugin_index, entry_index| {
            if entry_index > 0 {
                return (plugin_index, 0);
            }
            (plugin_index.saturating_sub(1), 0)
        });
    }

    fn select_next_plugin(&mut self) {
        self.select_position(|plugins, plugin_index, _| ((plugin_index + 1) % plugins.len(), 0));
    }

    fn register_plugin(&mut self, plugin: crate::model::Plugin) {
        drop(
            plugin
                .app_channel_out
                .send(crate::model::PluginRequest::Search(self.query.clone())),
        );
        self.plugins.push(plugin);
        self.plugins.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    fn update_entries(&mut self, plugin_id: String, entries: Vec<crate::model::Entry>) {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id);
        if plugin.is_none() {
            println!(
                "Appending entry failed. Could not find plugin with id {:?}",
                plugin_id
            );
            return;
        }

        let plugin = plugin.unwrap();
        plugin.entries = entries;
        self.normalize_selection();
    }

    fn activate_selected_entry(&mut self) -> Option<()> {
        let (plugin, entry) = self.active_entry()?;
        plugin
            .app_channel_out
            .send_blocking(model::PluginRequest::Activate(entry.clone()))
            .ok()
    }

    fn set_fonts(&mut self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "NerdFontSymbols".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/SymbolsNerdFontMono-Regular.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("NerdFontSymbols".to_owned());
        ctx.set_fonts(fonts);
    }

    fn set_theme(&mut self, ctx: &egui::Context) {
        let text_styles: std::collections::BTreeMap<_, _> = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(0.75 * crate::REM, egui::FontFamily::Monospace),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(1. * crate::REM, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());

        let settings = settings::Settings::get_or_init();
        ctx.set_visuals_of(
            egui::Theme::Dark,
            egui::Visuals {
                override_text_color: Some(settings::hexcolor(&settings.color.text)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();
        self.set_fonts(ctx);
        self.set_theme(ctx);
        self.handle_input(ctx);
        self.handle_messages();

        let settings = settings::Settings::get_or_init();

        egui::Frame::new()
            .inner_margin(egui::epaint::MarginF32 {
                bottom: 1. * crate::REM,
                ..Default::default()
            })
            .corner_radius(0.5 * crate::REM)
            .fill(settings::hexcolor(&settings.color.background))
            .show(ui, |ui| {
                let searchbar = component::query_input::view(ui, &mut self.query);
                searchbar.request_focus();
                if searchbar.changed() {
                    self.search();
                }
                ui.separator();

                let plugins = self.non_empty_plugins();
                let Some((active_plugin_index, active_entry_index)) =
                    self.active_position(&plugins)
                else {
                    return;
                };

                if active_entry_index > 0 {
                    let previous_entry =
                        &plugins[active_plugin_index].entries[active_entry_index - 1];
                    component::entry::view(ui, previous_entry, false);
                }

                for (plugin_index, plugin) in plugins.iter().enumerate().skip(active_plugin_index) {
                    if ui.available_height() < ENTRY_HEIGHT {
                        break;
                    }

                    let is_active_plugin = plugin_index == active_plugin_index;
                    let first_entry_index = if is_active_plugin {
                        active_entry_index
                    } else {
                        0
                    };

                    if first_entry_index == 0 {
                        component::plugin_header::view(ui, plugin, !is_active_plugin);
                    }

                    for (entry_index, entry) in
                        plugin.entries.iter().enumerate().skip(first_entry_index)
                    {
                        if ui.available_height() < ENTRY_HEIGHT {
                            break;
                        }

                        component::entry::view(
                            ui,
                            entry,
                            is_active_plugin && entry_index == active_entry_index,
                        );
                    }
                }
            });
    }
}

impl Centerpiece {}

pub const REM: f32 = 14.0;
pub const ENTRY_HEIGHT: f32 = 2.5 * crate::REM;
