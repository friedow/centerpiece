use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct ProgressiveWebAppsPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for ProgressiveWebAppsPlugin {
    fn id() -> &'static str {
        return "brave-progressive-web-apps";
    }

    fn priority() -> u32 {
        return 28;
    }

    fn title() -> &'static str {
        return "󰀻 Progressive Web Apps";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn new() -> Self {
        let bookmarks_root_result = crate::plugin::brave::utils::read_bookmarks_file();
        if let Err(error) = bookmarks_root_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }

        let bookmarks_root = bookmarks_root_result.unwrap();
        let folder_name = String::from("Progressive Web Apps");
        let pwa_folder_result = bookmarks_root
            .find_bookmarks_folder_recursive(&folder_name)
            .ok_or(anyhow::anyhow!(
                "Unable to find a bookmarks folder named '{}'.",
                folder_name
            ));

        if let Err(error) = pwa_folder_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }

        let entries = pwa_folder_result
            .unwrap()
            .get_bookmarks_recursive(&vec![])
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        return Self { entries };
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(format!("--app={}", entry_id))
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
