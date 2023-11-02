use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct ApplicationsPlugin {
    entries: Vec<ExtendedEntry>,
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

impl Plugin for ApplicationsPlugin {
    fn new() -> Self {
        return Self { entries: vec![] };
    }

    fn id() -> &'static str {
        return "applications";
    }

    fn priority() -> u32 {
        return 29;
    }

    fn title() -> &'static str {
        return "󰀻 Apps";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self
            .entries
            .iter()
            .map(|extended_entry| extended_entry.entry.clone())
            .collect();
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let paths =
            freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths());
        let desktop_entries: std::collections::HashSet<ExtendedEntry> = paths
            .filter_map(|path| {
                let desktop_entry_result = ExtendedEntry::try_from(&path);
                if let Err(error) = desktop_entry_result {
                    log::warn!(target: "applications", "Skipping desktop entry: '{:?}'.", error);
                    return None;
                }
                return desktop_entry_result.ok();
            })
            .collect();
        self.entries = desktop_entries.into_iter().collect();

        return Ok(());
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let entry = self
            .entries
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

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry_id
            ))?;

        return Ok(());
    }
}
