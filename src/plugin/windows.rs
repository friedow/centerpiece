use std::{format, vec};

use iced::futures::StreamExt;

pub struct WindowsPlugin {
    plugin: crate::model::Plugin,
    all_entries: Vec<crate::model::Entry>,
    matcher: nucleo::Matcher,
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
            log::warn!(error = log::as_error!(error); "Failed to establish sway ipc connection");
            panic!("Failed to establish sway ipc connection!");
        }
        let mut sway = connection_result.unwrap();

        let mut config = nucleo::Config::DEFAULT;
        config.prefer_prefix = true;
        let matcher = nucleo::Matcher::new(config);

        return WindowsPlugin {
            all_entries: WindowsPlugin::all_entries(&mut sway),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("windows"),
                priority: 30,
                title: String::from("󰖯 Windows"),
                app_channel_out,
                entries: vec![],
            },
            sway,
            matcher,
        };
    }

    fn all_entries(sway: &mut swayipc::Connection) -> Vec<crate::model::Entry> {
        let root_node_result = sway.get_tree();
        if let Err(error) = root_node_result {
            log::warn!(error = log::as_error!(error); "Failed to retrieve the root node");
            return vec![];
        }
        let root_node = root_node_result.unwrap();

        return WindowsPlugin::get_window_nodes(root_node)
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

    fn search(&mut self, query: &str) {
        // let filtered_entries = crate::plugin::utils::search(self.all_entries.clone(), query);
        let pattern = nucleo::pattern::Atom::new(
            query,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::AtomKind::Fuzzy,
            false,
        );
        let filtered_vec = pattern.match_list(self.all_entries.clone(), &mut self.matcher);
        let filtered_entries: Vec<crate::model::Entry> =
            filtered_vec.into_iter().map(|(e, _)| e).collect();

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
        let focus_cmd_result = self
            .sway
            .run_command(format!("[con_id={}] focus", entry_id));
        if let Err(error) = focus_cmd_result {
            log::warn!(error = log::as_error!(error); "Failed to focus window");
        }

        self.plugin_channel_out.try_send(crate::Message::Exit).ok();
    }
}
