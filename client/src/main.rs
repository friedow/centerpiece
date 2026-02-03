use std::process::exit;

use clap::Parser;
use eframe::egui::{self, Separator};

mod component;
mod model;
mod plugin;

pub fn main() {
    let args = settings::cli::CliArgs::parse();
    settings::Settings::try_from(args).unwrap_or_else(|_| {
        eprintln!("There is an issue with the settings, please check the configuration file.");
        std::process::exit(1);
    });

    simple_logger::init_with_level(log::Level::Info).unwrap();

    eframe::run_native(
        "centerpiece",
        settings(),
        Box::new(|cc| Ok(Box::new(Centerpiece::new(cc)))),
    );
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
    active_entry_index: usize,
    plugins: Vec<model::Plugin>,
    plugin_channels: Vec<async_std::channel::Receiver<Message>>,
}

impl eframe::App for Centerpiece {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.handle_input(ctx);

        let mut messages = vec![];

        for plugin_channel in &self.plugin_channels {
            let message_result = plugin_channel.try_recv();
            if message_result.is_err() {
                continue;
            }
            messages.push(message_result.unwrap());
        }

        self.handle_messages(messages);

        let settings = settings::Settings::get_or_init();

        eframe::egui::CentralPanel::default()
            .frame(eframe::egui::Frame::new())
            .show(ctx, |ui| {
                eframe::egui::Frame::new()
                    .inner_margin(eframe::egui::epaint::Marginf {
                        bottom: 1. * crate::REM,
                        ..Default::default()
                    })
                    .fill(settings::hexcolor(&settings.color.background))
                    .show(ui, |ui| {
                        let response = ui.add(
                            eframe::egui::TextEdit::singleline(&mut self.query)
                                .hint_text("Search")
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .frame(false)
                                .margin(eframe::egui::epaint::Marginf {
                                    left: 1. * crate::REM,
                                    right: 1. * crate::REM,
                                    top: 1. * crate::REM,
                                    bottom: 0.75 * crate::REM,
                                }),
                        );
                        response.request_focus();
                        if response.changed() {
                            self.search();
                        }

                        let entries = self.entries();
                        if !entries.is_empty() {
                            ui.add(Separator::default().spacing(0.));
                        }

                        let mut divider_added = true;
                        let mut header_added = false;
                        let mut next_entry_index_to_add = self.active_entry_index;
                        let mut lines_added = 0;

                        while ui.available_height() > 0. {
                            if next_entry_index_to_add >= entries.len() {
                                break;
                            }

                            let mut plugin_to_add = None;
                            let mut last_plugin_start_index = 0;
                            for plugin in self.plugins.iter() {
                                if last_plugin_start_index == next_entry_index_to_add {
                                    plugin_to_add = Some(plugin);
                                }
                                last_plugin_start_index += plugin.entries.len();
                            }

                            if !divider_added && plugin_to_add.is_some() {
                                component::divider::view(ui);
                                divider_added = true;
                                lines_added += 1;
                                continue;
                            }

                            if !header_added && plugin_to_add.is_some() {
                                component::plugin_header::view(ui, plugin_to_add.unwrap());
                                header_added = true;
                                lines_added += 1;
                                continue;
                            } else if lines_added == 0 {
                                component::entry::view(
                                    ui,
                                    entries[next_entry_index_to_add - 1],
                                    false,
                                );
                            }

                            component::entry::view(
                                ui,
                                entries[next_entry_index_to_add],
                                next_entry_index_to_add == self.active_entry_index,
                            );
                            divider_added = false;
                            header_added = false;
                            next_entry_index_to_add += 1;
                            lines_added += 1;
                        }
                    });
            });
    }
}

fn settings() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            window_level: Some(eframe::egui::WindowLevel::AlwaysOnTop),
            transparent: Some(true),
            ..Default::default()
        },
        centered: true,
        ..Default::default()
    }
}

impl Centerpiece {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            "NerdFontSymbols".to_owned(),
            std::sync::Arc::new(eframe::egui::FontData::from_static(include_bytes!(
                "../assets/SymbolsNerdFontMono-Regular.ttf"
            ))),
        );
        fonts
            .families
            .entry(eframe::egui::FontFamily::Monospace)
            .or_default()
            .push("NerdFontSymbols".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let text_styles: std::collections::BTreeMap<_, _> = [
            (
                eframe::egui::TextStyle::Heading,
                eframe::egui::FontId::new(0.75 * crate::REM, eframe::egui::FontFamily::Monospace),
            ),
            (
                eframe::egui::TextStyle::Body,
                eframe::egui::FontId::new(1. * crate::REM, eframe::egui::FontFamily::Monospace),
            ),
        ]
        .into();
        cc.egui_ctx
            .all_styles_mut(move |style| style.text_styles = text_styles.clone());

        let settings = settings::Settings::get_or_init();
        cc.egui_ctx.set_visuals_of(
            eframe::egui::Theme::Dark,
            eframe::egui::Visuals {
                override_text_color: Some(settings::hexcolor(&settings.color.text)),
                panel_fill: eframe::egui::Color32::TRANSPARENT,
                extreme_bg_color: eframe::egui::Color32::TRANSPARENT,
                code_bg_color: eframe::egui::Color32::TRANSPARENT,
                faint_bg_color: eframe::egui::Color32::TRANSPARENT,
                window_fill: eframe::egui::Color32::TRANSPARENT,
                ..Default::default()
            },
        );

        let mut centerpiece = Self::default();
        println!("creating centerpiece");
        centerpiece.launch_plugins();
        return centerpiece;
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

        if settings.plugin.sway_windows.enable {
            self.plugin_channels.push(crate::plugin::utils::spawn::<
                crate::plugin::sway_windows::SwayWindowsPlugin,
            >());
        }
    }

    fn handle_input(&mut self, ctx: &eframe::egui::Context) {
        if ctx.input(|i| i.key_pressed(eframe::egui::Key::ArrowUp)) {
            self.select_previous_entry();
        }
        if ctx.input(|i| i.key_pressed(eframe::egui::Key::ArrowDown)) {
            self.select_next_entry();
        }
        if ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
            self.activate_selected_entry();
        }
        if ctx.input(|i| i.key_pressed(eframe::egui::Key::Escape)) {
            exit(0);
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(eframe::egui::Key::J)) {
            self.select_next_entry();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(eframe::egui::Key::K)) {
            self.select_previous_entry();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(eframe::egui::Key::P)) {
            self.select_previous_plugin();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(eframe::egui::Key::N)) {
            self.select_next_plugin();
        }
    }

    fn handle_messages(&mut self, messages: Vec<Message>) {
        for message in messages {
            match message {
                Message::RegisterPlugin(plugin) => self.register_plugin(plugin),

                Message::UpdateEntries(plugin_id, entries) => {
                    self.update_entries(plugin_id, entries)
                }

                Message::Exit => exit(0),
            }
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

    fn search(&mut self) {
        for plugin in self.plugins.iter_mut() {
            let _ = plugin
                .app_channel_out
                .send_blocking(crate::model::PluginRequest::Search(self.query.clone()));
        }

        self.select_first_entry();
    }

    fn select_first_entry(&mut self) {
        self.active_entry_index = 0;
    }

    fn select_previous_entry(&mut self) {
        let entries = self.entries();
        if entries.is_empty() {
            return self.select_first_entry();
        }

        if self.active_entry_index == 0 {
            self.active_entry_index = entries.len() - 1;
            return;
        }

        self.active_entry_index -= 1;
    }

    fn select_next_entry(&mut self) {
        let entries = self.entries();
        if entries.is_empty() || self.active_entry_index == entries.len() - 1 {
            return self.select_first_entry();
        }

        self.active_entry_index += 1;
    }

    fn select_next_plugin(&mut self) {
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
    }

    fn select_previous_plugin(&mut self) {
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
    }

    fn register_plugin(&mut self, plugin: crate::model::Plugin) {
        let _ = plugin
            .app_channel_out
            .send(crate::model::PluginRequest::Search(self.query.clone()));
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
    }

    fn activate_selected_entry(&mut self) -> Option<()> {
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
            .send_blocking(model::PluginRequest::Activate(entry))
            .ok()
    }
}

pub const REM: f32 = 14.0;
pub const ENTRY_HEIGHT: f32 = 2.3 * crate::REM;
