use anyhow::Context;
use iced::futures::StreamExt;

pub struct WindowsPlugin {
    plugin: crate::model::Plugin,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
    sway: swayipc::Connection,
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

        let connection_result = swayipc::Connection::new();
        if let Err(error) = connection_result {
            log::error!(
                target: "windows",
                "{}", error,
            );
            std::process::exit(1);
        }
        let mut sway = connection_result.unwrap();

        let entries_result = WindowsPlugin::all_entries(&mut sway);
        if let Err(error) = entries_result {
            log::error!(
                target: "windows",
                "{}", error,
            );
            std::process::exit(1);
        }
        let entries = entries_result.unwrap();

        return WindowsPlugin {
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("windows"),
                priority: 30,
                title: String::from("󰖯 Windows"),
                app_channel_out,
                entries,
            },
            sway,
        };
    }

    fn all_entries(sway: &mut swayipc::Connection) -> anyhow::Result<Vec<crate::model::Entry>> {
        let root_node = sway.get_tree()?;

        let entries = WindowsPlugin::get_window_nodes(root_node)
            .into_iter()
            .map(|node| {
                let name = node
                    .name
                    .unwrap_or(String::from("-- window name missing --"));
                let app_id = node
                    .app_id
                    .unwrap_or(String::from("-- window app_id missing --"));
                let title = if name != "" { name } else { app_id };
                return crate::model::Entry {
                    id: node.id.to_string(),
                    title,
                    action: String::from("focus"),
                    meta: String::from("windows"),
                };
            })
            .collect();

        return Ok(entries);
    }

    fn get_window_nodes(node: swayipc::Node) -> Vec<swayipc::Node> {
        if !node.nodes.is_empty() {
            return node
                .nodes
                .into_iter()
                .flat_map(|n| WindowsPlugin::get_window_nodes(n))
                .collect();
        }

        if node.node_type == swayipc::NodeType::Con {
            return vec![node];
        }

        return vec![];
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.plugin.id.as_str(),
                "{}", error,
            );
            std::process::exit(1);
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.plugin.id.as_str(),
                    "{}", error,
                );
            }
        }
    }

    fn register_plugin(&mut self) -> anyhow::Result<()> {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .context("Failed to send message to register plugin.")?;

        return Ok(());
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query)?,
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id)?,
        }

        return Ok(());
    }

    fn search(&mut self, query: &String) -> anyhow::Result<()> {
        let filtered_entries = crate::plugin::utils::search(self.plugin.entries.clone(), query);

        self.plugin_channel_out
            .try_send(crate::Message::Clear(self.plugin.id.clone()))
            .context(format!(
                "Failed to send message to clear entries while searching for '{}'.",
                query
            ))?;

        for entry in filtered_entries {
            let entry_id = entry.id.clone();
            self.plugin_channel_out
                .try_send(crate::Message::AppendEntry(self.plugin.id.clone(), entry))
                .context(format!(
                    "Failed to send message to append the entry with '{}' while searching for '{}'.",
                    entry_id,
                    query
                ))?;
        }

        return Ok(());
    }

    fn activate(&mut self, entry_id: String) -> anyhow::Result<()> {
        self.sway
            .run_command(format!("[con_id={}] focus", entry_id))
            .context(format!(
                "Failed to focus window while activating entry with id '{}'.",
                entry_id
            ))?;

        self.plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry_id
            ))?;

        return Ok(());
    }
}
