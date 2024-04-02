use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct BookmarksPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for BookmarksPlugin {
    fn id() -> &'static str {
        "brave_bookmarks"
    }

    fn priority() -> u32 {
        25
    }

    fn title() -> &'static str {
        "󰃃 Bookmarks"
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
        self.entries = crate::plugin::brave::utils::read_bookmarks_file()?
            .get_bookmarks_recursive(&vec![String::from("Progressive Web Apps")])
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        Ok(())
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

        Ok(())
    }
}
