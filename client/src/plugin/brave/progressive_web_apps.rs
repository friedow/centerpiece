use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct ProgressiveWebAppsPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for ProgressiveWebAppsPlugin {
    fn id() -> &'static str {
        "brave_progressive_web_apps"
    }

    fn priority() -> u32 {
        28
    }

    fn title() -> &'static str {
        "󰀻 Progressive Web Apps"
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

        let folder_name = String::from("Progressive Web Apps");
        let bookmarks_root = crate::plugin::brave::utils::read_bookmarks_file()?;
        let pwa_folder = bookmarks_root
            .find_bookmarks_folder_recursive(&folder_name)
            .ok_or(anyhow::anyhow!(
                "Unable to find a bookmarks folder named '{}'.",
                folder_name
            ))?;
        self.entries = pwa_folder
            .get_bookmarks_recursive(&vec![])
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        self.sort();
        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(format!("--app={}", entry.id))
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
