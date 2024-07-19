use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct ApplicationsPlugin {
    entries: Vec<crate::model::Entry>,
}

fn to_entry(
    desktop_entry: &freedesktop_desktop_entry::DesktopEntry,
    terminal_command: Option<String>,
) -> Option<crate::model::Entry> {
    let title = name(desktop_entry);

    if !is_visible(desktop_entry) {
        log::debug!(target: "applications", "Desktop entry with name '{}' will be hidden because of its properties.", title);
        return None;
    }

    let mut cmd: Vec<String> = desktop_entry
        .exec()?
        .split_ascii_whitespace()
        .filter_map(|s| {
            if s.starts_with('%') {
                None
            } else {
                Some(String::from(s))
            }
        })
        .collect();

    if desktop_entry.terminal() {
        if terminal_command.is_none() {
            log::warn!(target: "applications", "Desktop entry with name '{}' will be hidden because no terminal emulator was found to launch it with.", title);
            return None;
        }

        cmd.insert(0, terminal_command.unwrap());
        cmd.insert(1, "-e".into());
    }

    let mut meta = desktop_entry
        .keywords()
        .unwrap_or(std::borrow::Cow::from(""))
        .replace(';', " ");
    meta.push_str(" Applications Apps");

    Some(crate::model::Entry {
        id: desktop_entry.appid.to_string(),
        title,
        action: String::from("open"),
        meta,
        command: Some(cmd),
    })
}

fn is_visible(desktop_entry: &freedesktop_desktop_entry::DesktopEntry) -> bool {
    if desktop_entry.type_() != Some("Application") {
        return false;
    }

    if desktop_entry.desktop_entry("Hidden") == Some("true") {
        return false;
    }

    if desktop_entry.no_display() {
        return false;
    }

    // filter entries where Exec == false
    if let Some(exec) = desktop_entry.exec() {
        if exec.to_ascii_lowercase() == "false" {
            return false;
        }
    }

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

        if !only_show_in_desktops.split(';').all(|d| d != desktop) {
            return false;
        }
    }

    true
}

fn terminal_command(desktop_entry: &freedesktop_desktop_entry::DesktopEntry) -> Option<String> {
    if !desktop_entry
        .categories()?
        .split(';')
        .any(|category| category == "TerminalEmulator")
    {
        return None;
    }
    return desktop_entry
        .exec()?
        .split_ascii_whitespace()
        .nth(0)
        .map(String::from);
}

fn name(desktop_entry: &freedesktop_desktop_entry::DesktopEntry) -> String {
    let locale = std::env::var("LANG").unwrap_or(String::from("en_US"));
    desktop_entry
        .name(Some(&locale))
        .unwrap_or_default()
        .to_string()
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

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let paths: Vec<std::path::PathBuf> =
            freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths())
                .collect();

        let bytes_collection: Vec<(&std::path::PathBuf, String)> = paths
            .iter()
            .filter_map(|path| Some((path, std::fs::read_to_string(path).ok()?)))
            .collect();

        let mut desktop_entries: Vec<freedesktop_desktop_entry::DesktopEntry> = bytes_collection
            .iter()
            .filter_map(|(bytes, path)| {
                freedesktop_desktop_entry::DesktopEntry::decode(bytes, path).ok()
            })
            .collect();

        desktop_entries.sort_by_key(name);
        desktop_entries.dedup_by_key(|desktop_entry| name(desktop_entry));

        let terminal_command = desktop_entries.iter().find_map(terminal_command);

        self.entries = desktop_entries
            .iter()
            .filter_map(|path| to_entry(path, terminal_command.clone()))
            .collect();

        self.sort();

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
