use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct ApplicationsPlugin {
    entries: Vec<crate::model::Entry>,
}

fn read_desktop_entry(path: &std::path::PathBuf) -> anyhow::Result<crate::model::Entry> {
    let pathstr = path.to_str().unwrap_or("");
    let bytes = std::fs::read_to_string(path)?;
    let desktop_entry = freedesktop_desktop_entry::DesktopEntry::decode(path, &bytes)?;

    if !is_visible(&desktop_entry) {
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
            if s.starts_with('%') {
                None
            } else {
                Some(String::from(s))
            }
        })
        .collect();

    let mut meta = desktop_entry
        .keywords()
        .unwrap_or(std::borrow::Cow::from(""))
        .replace(';', " ");
    meta.push_str(" Applications Apps");

    Ok(crate::model::Entry {
        id: desktop_entry.appid.to_string(),
        title,
        action: String::from("open"),
        meta,
        command: Some(cmd),
    })
}

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

    true
}

impl Plugin for ApplicationsPlugin {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn id() -> &'static str {
        "applications"
    }

    fn priority() -> u32 {
        29
    }

    fn title() -> &'static str {
        "󰀻 Apps"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let paths =
            freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths());
        self.entries = paths
            .filter_map(|path| {
                let desktop_entry_result = read_desktop_entry(&path);
                if let Err(error) = desktop_entry_result {
                    log::warn!(target: "applications", "Skipping desktop entry: '{:?}'.", error);
                    return None;
                }
                desktop_entry_result.ok()
            })
            .collect();

        self.entries.sort();
        self.entries.dedup();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let command = entry.command.context(format!(
            "Failed to unpack command while activating entry with id '{}'.",
            entry.id
        ))?;
        std::process::Command::new(&command[0])
            .args(&command[1..])
            .spawn()?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
