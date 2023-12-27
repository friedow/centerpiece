use crate::plugin::utils::Plugin;
use anyhow::Context;

use sqlite::State;

pub struct HistoryPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for HistoryPlugin {
    fn id() -> &'static str {
        return "brave-history";
    }

    fn priority() -> u32 {
        return 99;
    }

    fn title() -> &'static str {
        return "󰃃 History";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn new() -> Self {
        println!("------------------------------------------ HELLOOOOOOOOOO");

        let home_directory = std::env::var("HOME").unwrap();

        let history_file_path = std::path::Path::new(&home_directory)
            .join(".config/BraveSoftware/Brave-Browser/Default/History");

        // TODO: database is locked, needs to becopied first
        let connection = sqlite::open(history_file_path).unwrap();

        let query = "SELECT * FROM urls";
        connection.execute(query).unwrap();
        let url_rows = connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        for url_row in url_rows {
            println!("title = {}", url_row.read::<&str, _>("title"));
            println!("url = {}", url_row.read::<&str, _>("url"));
        }

        return Self { entries: vec![] };
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        // TODO: add entries here

        return Ok(());
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch brave in app mode while activating entry with id '{}'.",
                entry.id
            ))?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        return Ok(());
    }
}
