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
                .flat_map(|n| Self::get_window_nodes(n))
                .collect();
        }

        if node.node_type == swayipc::NodeType::Con {
            return vec![node];
        }

        return vec![];
    }
}

impl Plugin for WindowsPlugin {
    fn id() -> &'static str {
        return "windows";
    }
    fn priority() -> u32 {
        return 30;
    }
    fn title() -> &'static str {
        return "󰖯 Windows";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
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
                let title = if name != "" { name } else { app_id };
                return crate::model::Entry {
                    id: node.id.to_string(),
                    title,
                    action: String::from("focus"),
                    meta: String::from(Self::id()),
                };
            })
            .collect();

        return Self { sway, entries };
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        self.sway
            .run_command(format!("[con_id={}] focus", entry_id))
            .context(format!(
                "Failed to focus window while activating entry with id '{}'.",
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
