use anyhow::{Context, Result};

use crate::plugin::utils::Plugin;

pub struct MpdPlayerPlugin {
    entries: Vec<crate::model::Entry>,
    settings: crate::settings::Settings,
}

impl MpdPlayerPlugin {
    fn get_playlist_entries(&self) -> Result<Vec<crate::model::Entry>> {
        let mut conn = mpd::Client::connect("127.0.0.1:6600").unwrap();
        let playlists = conn.playlists().unwrap();
        let entries: Vec<_> = playlists
            .iter()
            .map(|playlist| crate::model::Entry {
                id: playlist.name.clone(),
                title: playlist.name.clone(),
                action: String::from("focus"),
                meta: String::from("mpd music playlist"),
                command: Some(vec![
                    String::from("mpc"),
                    String::from("load"),
                    playlist.name.clone(),
                ]),
            })
            .collect();
        let songs = conn.listall().unwrap();
        let entries: Vec<_> = songs
            .iter()
            .map(|song| crate::model::Entry {
                id: song.file.clone(),
                title: song.file.clone(),
                action: String::from("focus"),
                meta: String::from("mpd music song"),
                command: Some(vec![
                    String::from("mpc"),
                    String::from("play"),
                    song.file.clone(),
                ]),
            })
            .collect();
        Ok(entries)
    }
}

impl Plugin for MpdPlayerPlugin {
    fn new() -> Self {
        Self {
            entries: vec![],
            settings: crate::settings::Settings::new(),
        }
    }

    fn id() -> &'static str {
        "mpd player"
    }

    fn priority() -> u32 {
        26
    }

    fn title() -> &'static str {
        " Music"
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
