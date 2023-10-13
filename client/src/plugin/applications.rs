use anyhow::Context;
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

impl TryFrom<&std::path::PathBuf> for ExtendedEntry {
    type Error = anyhow::Error;

    fn try_from(path: &std::path::PathBuf) -> anyhow::Result<ExtendedEntry> {
        let pathstr = path.to_str().unwrap_or("");
        let bytes = std::fs::read_to_string(path)?;
        let desktop_entry = freedesktop_desktop_entry::DesktopEntry::decode(&path, &bytes)?;

        if !ExtendedEntry::is_visible(&desktop_entry) {
            return Err(anyhow::anyhow!(
                "Desktop entry at path '{}' is hidden.",
                pathstr
            ));
        }

        let locale = std::env::var("LANG").unwrap_or(String::from("en_US"));
        let title = desktop_entry
            .name(Some(&locale))
            .context(format!(
                "Desktop entry at path '{}' is missing the 'name' field.",
                pathstr
            ))?
            .to_string();

        let cmd = desktop_entry
            .exec()
            .context(format!(
                "Desktop entry at path '{}' is missing the 'exec' field.",
                pathstr
            ))?
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
                title,
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
                    log::warn!(target: "applications", "Skipping desktop entry: '{}'.", error);
                    return None;
                }
                return desktop_entry_result.ok();
            })
            .collect();
        return desktop_entries.into_iter().collect();
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.plugin.id.as_str(),
                "{}", error
            );
            std::process::exit(1);
        }

        let search_result = self.search(&String::from(""));
        if let Err(error) = search_result {
            log::warn!(
                target: self.plugin.id.as_str(),
                "{}", error
            );
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    "{}", error
                );
            }
        }
    }

    fn register_plugin(&mut self) -> anyhow::Result<()> {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .context("Failed to send message to register plugin.")?;

        return Ok(());
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query)?,
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id)?,
        }

        return Ok(());
    }

    fn search(&mut self, query: &String) -> anyhow::Result<()> {
        let all_entries = self
            .all_entries
            .clone()
            .into_iter()
            .map(|extended_entry| extended_entry.entry)
            .collect();
        let filtered_entries = crate::plugin::utils::search(all_entries, query);

        self.plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                self.plugin.id.clone(),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries while searching for '{}'.",
                query
            ))?;

        return Ok(());
    }

    fn activate(&mut self, entry_id: String) -> anyhow::Result<()> {
        let entry = self
            .all_entries
            .iter()
            .find(|e| e.entry.id == entry_id)
            .context(format!(
                "Failed to find entry with id '{}' while activating it.",
                entry_id
            ))?;

        let cmd = entry.cmd.clone();

        std::process::Command::new(cmd[0].clone())
            .args(&cmd[1..])
            .spawn()
            .context(format!(
                "Failed to launch application while activating entry with id '{}'.",
                entry_id
            ))?;

        self.plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry_id
            ))?;

        return Ok(());
    }
}
