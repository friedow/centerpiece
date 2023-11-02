use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct BookmarksPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for BookmarksPlugin {
    fn id() -> &'static str {
        return "brave-bookmarks";
    }

    fn priority() -> u32 {
        return 25;
    }

    fn title() -> &'static str {
        return "󰃃 Bookmarks";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn new() -> Self {
        return Self { entries: vec![] };
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        self.entries = crate::plugin::brave::utils::read_bookmarks_file()?
            .get_bookmarks_recursive(&vec![String::from("Progressive Web Apps")])
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        return Ok(());
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch brave in app mode while activating entry with id '{}'.",
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
