use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct SwayWindowsPlugin {
    sway: swayipc::Connection,
    entries: Vec<crate::model::Entry>,
}

impl SwayWindowsPlugin {
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

impl Plugin for SwayWindowsPlugin {
    fn id() -> &'static str {
        "sway-windows"
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

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        let connection_result =
            swayipc::Connection::new().context("Failed to establish sway ipc connection.");
        if let Err(error) = connection_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }
        let sway = connection_result.unwrap();

        Self {
            sway,
            entries: vec![],
        }
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        let root_node_result = self
            .sway
            .get_tree()
            .context("Failed to get_tree from sway ipc.");
        if let Err(error) = root_node_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }
        let sway_root_node = root_node_result.unwrap();

        let entries: Vec<crate::model::Entry> = Self::get_window_nodes(sway_root_node)
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
                    meta: String::from("Sway Windows"),
                    command: None,
                }
            })
            .collect();

        self.set_entries(entries);
        self.sort();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut async_std::channel::Sender<crate::Message>,
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
