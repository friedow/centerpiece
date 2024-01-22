use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct WindowsPlugin {
    sway: swayipc::Connection,
    entries: Vec<crate::model::Entry>,
}

impl WindowsPlugin {
    fn get_window_nodes(node: swayipc::Node) -> Vec<swayipc::Node> {
        if !node.nodes.is_empty() {
            return node
                .nodes
                .into_iter()
                .flat_map(Self::get_window_nodes)
                .collect();
        }

        if node.node_type == swayipc::NodeType::Con {
            return vec![node];
        }

        vec![]
    }
}

impl Plugin for WindowsPlugin {
    fn id() -> &'static str {
        "windows"
    }
    fn priority() -> u32 {
        30
    }
    fn title() -> &'static str {
        "󰖯 Windows"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn new() -> Self {
        let connection_result =
            swayipc::Connection::new().context("Failed to establish sway ipc connection.");
        if let Err(error) = connection_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }
        let mut sway = connection_result.unwrap();

        let root_node_result = sway.get_tree().context("Failed to get_tree from sway ipc.");
        if let Err(error) = root_node_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }
        let root_node = root_node_result.unwrap();

        let entries = Self::get_window_nodes(root_node)
            .into_iter()
            .map(|node| {
                let name = node
                    .name
                    .unwrap_or(String::from("-- window name missing --"));
                let app_id = node
                    .app_id
                    .unwrap_or(String::from("-- window app_id missing --"));
                let title = if !name.is_empty() { name } else { app_id };
                crate::model::Entry {
                    id: node.id.to_string(),
                    title,
                    action: String::from("focus"),
                    meta: String::from(Self::id()),
                    command: None,
                }
            })
            .collect();

        Self { sway, entries }
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        self.sway
            .run_command(format!("[con_id={}] focus", entry.id))
            .context(format!(
                "Failed to focus window while activating entry with id '{}'.",
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
