use std::fs;

use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct TabsPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for TabsPlugin {
    fn id() -> &'static str {
        "firefox_tabs"
    }

    fn priority() -> u32 {
        30
    }

    fn title() -> &'static str {
        "󰋚 Browser Tabs"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        let mut entries = vec![];

        let profile_path = crate::plugin::firefox::utils::profile_path()?;
        let session_file_path = format!("{profile_path}/sessionstore-backups/recovery.jsonlz4");

        // The first 8 bytes of the file have to be ignored since they are part
        // of mozillas custom file format which is not 100% compatible with lz4.
        // https://unix.stackexchange.com/questions/385023/firefox-reading-out-urls-of-opened-tabs-from-the-command-line#comment745179_389360
        let session_file_raw = fs::read(&session_file_path).unwrap().split_off(8);
        let session_file_decompressed =
            lz4_flex::decompress_size_prepended(&session_file_raw).unwrap();
        let session_file: serde_json::Value =
            serde_json::from_slice(&session_file_decompressed).unwrap();

        let windows = match &session_file["windows"] {
            serde_json::Value::Array(array) => array.clone(),
            _ => vec![],
        };
        for window in windows.iter() {
            let tabs = match &window["tabs"] {
                serde_json::Value::Array(array) => array.clone(),
                _ => vec![],
            };
            for tab in tabs {
                let history_entries = match &tab["entries"] {
                    serde_json::Value::Array(array) => array.clone(),
                    _ => vec![],
                };
                let tab_info = history_entries.last().unwrap();

                let url = match &tab_info["url"] {
                    serde_json::Value::String(url) => url.clone(),
                    _ => "Tab without Title".into(),
                };

                let title = match &tab_info["title"] {
                    serde_json::Value::String(title) => title.clone(),
                    _ => "Tab without Title".into(),
                };

                entries.push(crate::model::Entry {
                    id: url,
                    title,
                    action: String::from("open"),
                    meta: String::from("History"),
                    command: None,
                });
            }
        }

        self.set_entries(entries);

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("firefox")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch firefox while activating entry with id '{}'.",
                entry.id
            ))?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
