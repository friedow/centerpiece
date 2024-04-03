use anyhow::{Context, Result};

use crate::plugin::utils::Plugin;

pub struct MpdPlaylistPlugin {
    entries: Vec<crate::model::Entry>,
    settings: crate::settings::Settings,
    connection: mpd::Client,
}

impl MpdPlaylistPlugin {
    fn get_playlist_entries(&mut self) -> Result<Vec<crate::model::Entry>> {
        let playlists = self.connection.playlists().unwrap();
        let entries: Vec<_> = playlists
            .iter()
            .map(|playlist| crate::model::Entry {
                id: playlist.name.clone(),
                title: playlist.name.clone(),
                action: String::from("load"),
                meta: String::from("mpd music playlist"),
                command: None,
            })
            .collect();
        Ok(entries)
    }
}

impl Plugin for MpdPlaylistPlugin {
    fn new() -> Self {
        let conn = mpd::Client::connect("127.0.0.1:6600").unwrap();
        Self {
            entries: vec![],
            settings: crate::settings::Settings::new(),
            connection: conn,
        }
    }

    fn id() -> &'static str {
        "mpd playlists"
    }

    fn priority() -> u32 {
        24
    }

    fn title() -> &'static str {
        "󰲸 Playlists"
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        self.entries = self.get_playlist_entries()?;
        Ok(())
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        log::warn!("entry: {:?}", entry);
        self.connection.clear().context("Failed to clear queue.")?;
        self.connection
            .load(&entry.id, ..)
            .context(format!("Failed to load playlist with id '{}'.", entry.id))?;

        self.connection.play().unwrap();

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
