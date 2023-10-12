use iced::futures::StreamExt;

pub struct ApplicationsPlugin {
    plugin: crate::model::Plugin,
    all_entries: Vec<ExtendedEntry>,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

#[derive(Debug, Clone)]
struct ExtendedEntry {
    cmd: Vec<String>,
    entry: crate::model::Entry,
}

impl Eq for ExtendedEntry {}

impl PartialEq for ExtendedEntry {
    fn eq(&self, other: &Self) -> bool {
        return self.entry.id == other.entry.id;
    }
}

impl std::hash::Hash for ExtendedEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entry.id.hash(state);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("Failed to read desktop entry file.")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to decode desktop entry file.")]
    DecodeError(#[from] freedesktop_desktop_entry::DecodeError),
    #[error("Desktop entry is hidden.")]
    IsHidden,
    #[error("Desktop entry is missing a name field.")]
    MissingName,
    #[error("Desktop entry is missing an exec field.")]
    MissingExec,
}

impl TryFrom<&std::path::PathBuf> for ExtendedEntry {
    type Error = ParsingError;

    fn try_from(path: &std::path::PathBuf) -> Result<ExtendedEntry, ParsingError> {
        let bytes = std::fs::read_to_string(path)?;
        let desktop_entry = freedesktop_desktop_entry::DesktopEntry::decode(&path, &bytes)?;

        if !ExtendedEntry::is_visible(&desktop_entry) {
            return Err(ParsingError::IsHidden);
        }

        let locale = std::env::var("LANG").unwrap_or(String::from("en_US"));
        let name_option = desktop_entry.name(Some(&locale));
        if name_option.is_none() {
            return Err(ParsingError::MissingName);
        }

        let exec_option = desktop_entry.exec();
        if exec_option.is_none() {
            return Err(ParsingError::MissingExec);
        }

        let cmd = exec_option
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|s| {
                if s.starts_with("%") {
                    None
                } else {
                    Some(String::from(s))
                }
            })
            .collect();

        let mut meta = desktop_entry
            .keywords()
            .unwrap_or(std::borrow::Cow::from(""))
            .replace(";", " ");
        meta.push_str(" Applications Apps");

        return Ok(ExtendedEntry {
            cmd,
            entry: crate::model::Entry {
                id: desktop_entry.appid.to_string(),
                title: name_option.unwrap().to_string(),
                action: String::from("open"),
                meta,
            },
        });
    }
}

impl ExtendedEntry {
    fn is_visible(desktop_entry: &freedesktop_desktop_entry::DesktopEntry) -> bool {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or(String::from("sway"));
        // filter entries where NotShowIn == current desktop
        if let Some(not_show_in) = desktop_entry.desktop_entry("NotShowIn") {
            let not_show_in_desktops = not_show_in.to_ascii_lowercase();

            if not_show_in_desktops.split(';').any(|d| d == desktop) {
                return false;
            }
        }

        // filter entries where OnlyShowIn != current desktop
        if let Some(only_show_in) = desktop_entry.only_show_in() {
            let only_show_in_desktops = only_show_in.to_ascii_lowercase();

            if !only_show_in_desktops.split(';').any(|d| d == desktop) {
                return false;
            }
        }

        // filter entries where NoDisplay != true
        if desktop_entry.no_display() {
            return false;
        }

        // filter entries where Exec == false
        if let Some(exec) = desktop_entry.exec() {
            if exec.to_ascii_lowercase() == "false" {
                return false;
            }
        }

        return true;
    }
}

impl ApplicationsPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<ApplicationsPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = ApplicationsPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> ApplicationsPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return ApplicationsPlugin {
            all_entries: ApplicationsPlugin::all_entries(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("applications"),
                priority: 29,
                title: String::from("󰀻 Apps"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    fn all_entries() -> Vec<ExtendedEntry> {
        let paths =
            freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths());
        let desktop_entries: std::collections::HashSet<ExtendedEntry> = paths
            .filter_map(|path| {
                let desktop_entry_result = ExtendedEntry::try_from(&path);
                if let Err(error) = desktop_entry_result {
                    log::warn!(target: "applications", error = log::as_error!(error); "Skipping desktop entry with path '{}'.", &path.to_str().unwrap());
                    return None;
                }
                return desktop_entry_result.ok();
            })
            .collect();
        return desktop_entries.into_iter().collect();
    }

    async fn main(&mut self) -> ! {
        self.register_plugin();
        self.search(&String::from(""));

        loop {
            self.update().await;
        }
    }

    fn register_plugin(&mut self) {
        let send_register_plugin_result = self
            .plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()));
        if let Err(error) = send_register_plugin_result {
            log::error!(
                error = log::as_error!(error);
                "Failed to send message to register the plugin.",
            );
            std::process::exit(1);
        }
    }

    async fn update(&mut self) {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query),
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id),
        }
    }

    fn search(&mut self, query: &String) {
        let all_entries = self
            .all_entries
            .clone()
            .into_iter()
            .map(|extended_entry| extended_entry.entry)
            .collect();
        let filtered_entries = crate::plugin::utils::search(all_entries, query);

        let send_clear_entries_result = self
            .plugin_channel_out
            .try_send(crate::Message::Clear(self.plugin.id.clone()));
        if let Err(error) = send_clear_entries_result {
            log::warn!(
                target: self.plugin.id.as_str(),
                error = log::as_error!(error);
                "Failed to send message to clear all entries.",
            );
        }

        for entry in filtered_entries {
            let entry_id = entry.id.clone();
            let send_append_entry_result = self
                .plugin_channel_out
                .try_send(crate::Message::AppendEntry(self.plugin.id.clone(), entry));
            if let Err(error) = send_append_entry_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    error = log::as_error!(error);
                    "Failed to send message to append entry with id '{}'.", &entry_id
                );
            }
        }
    }

    fn activate(&mut self, entry_id: String) {
        let entry_option = self.all_entries.iter().find(|e| e.entry.id == entry_id);
        if entry_option.is_none() {
            log::warn!(
                target: self.plugin.id.as_str(),
                "Failed to activate entry with id '{}'.",
                entry_id
            );
            return;
        }

        let cmd = entry_option.unwrap().cmd.clone();

        std::process::Command::new(cmd[0].clone())
            .args(&cmd[1..])
            .spawn()
            .expect("Command failure");

        let send_exit_result = self.plugin_channel_out.try_send(crate::Message::Exit);
        if let Err(error) = send_exit_result {
            log::warn!(
                target: self.plugin.id.as_str(),
                error = log::as_error!(error);
                "Failed to send message to exit the application.",
            );
        }
    }
}