use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct HistoryPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for HistoryPlugin {
    fn id() -> &'static str {
        return "brave-history";
    }

    fn priority() -> u32 {
        return 0;
    }

    fn title() -> &'static str {
        return "󰃃 History";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn new() -> Self {
        return Self { entries: vec![] };
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let home_directory =
            std::env::var("HOME").context("Could not read HOME environment variable")?;

        let cache_directory_path = std::path::Path::new(&home_directory).join(".cache/centerpiece");
        std::fs::create_dir_all(&cache_directory_path)
            .context("Error while creating cache directory")?;

        let history_file_path = std::path::Path::new(&home_directory)
            .join(".config/BraveSoftware/Brave-Browser/Default/History");
        let history_cache_file_path = cache_directory_path.join("brave-history.sqlite");

        std::fs::copy(&history_file_path, &history_cache_file_path)
            .context("Error while creating cache directory")?;

        let connection = sqlite::open(history_cache_file_path).unwrap();

        let query = "SELECT title, url FROM urls ORDER BY visit_count DESC, last_visit_time DESC";
        connection.execute(query).unwrap();
        let url_rows = connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        self.entries = url_rows
            .map(|row| {
                let title = row.read::<&str, _>("title");
                let url = row.read::<&str, _>("url");

                return crate::model::Entry {
                    id: url.to_string(),
                    title: title.to_string(),
                    action: String::from("open"),
                    meta: String::from("History"),
                    command: None,
                };
            })
            .collect();

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
                "Failed to launch brave while activating entry with id '{}'.",
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
