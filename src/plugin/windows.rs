use std::vec;

use iced::futures::StreamExt;

pub struct WindowsPlugin {
    plugin: crate::model::Plugin,
    all_entries: Vec<crate::model::Entry>,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}


#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("unable to read desktop file")]
    ReadError(#[from] std::io::Error),
    #[error("unable to decode desktop file")]
    DecodeError(#[from] freedesktop_desktop_entry::DecodeError),
    #[error("desktop entry is hidden")]
    IsHidden,
    #[error("desktop entry is missing a name")]
    MissingName,
    #[error("desktop entry is missing an exec")]
    MissingExec,
}

impl WindowsPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<WindowsPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = WindowsPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> WindowsPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return WindowsPlugin {
            all_entries: WindowsPlugin::all_entries(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("windows"),
                priority: 0,
                title: String::from(" Windows"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    fn all_entries() -> Vec<crate::model::Entry> {
        use std::io::{stdin, stdout, Write};
        let mut connection_result = swayipc::Connection::new();
        if let Err(error) = connection_result {
            log::warn!(error = log::as_error!(error); "Failed to establish sway ipc connection");
            return vec![];
        }
        let mut connection = connection_result.unwrap();

    }

    async fn main(&mut self) -> ! {
        self.register_plugin();
        self.search(&String::from(""));

        loop {
            self.update().await;
        }
    }

    fn register_plugin(&mut self) {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .ok();
    }

    async fn update(&mut self) {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query),
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id),
        }
    }

    fn search(&mut self, query: &String) {
        let all_entries = self
            .all_entries
            .clone()
            .into_iter()
            .map(|extended_entry| extended_entry.entry)
            .collect();
        let filtered_entries = crate::plugin::utils::search(all_entries, query);

        self.plugin_channel_out
            .try_send(crate::Message::Clear(self.plugin.id.clone()))
            .ok();

        for entry in filtered_entries {
            self.plugin_channel_out
                .try_send(crate::Message::AppendEntry(self.plugin.id.clone(), entry))
                .ok();
        }
    }

    fn activate(&mut self, entry_id: String) {
        let entry_option = self.all_entries.iter().find(|e| e.entry.id == entry_id);
        if entry_option.is_none() {
            log::warn!(
                "Entry activation failed: Unable to find entry with id {}.",
                entry_id
            );
            return;
        }

        let cmd = entry_option.unwrap().cmd.clone();

        std::process::Command::new(cmd[0].clone())
            .args(&cmd[1..])
            .spawn()
            .expect("Command failure");

        self.plugin_channel_out.try_send(crate::Message::Exit).ok();
    }
}
