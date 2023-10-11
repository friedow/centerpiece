use iced::futures::StreamExt;

pub struct GitRepositoriesPlugin {
    plugin: crate::model::Plugin,
    all_entries: Vec<crate::model::Entry>,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

impl GitRepositoriesPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<GitRepositoriesPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = GitRepositoriesPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> GitRepositoriesPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return GitRepositoriesPlugin {
            all_entries: GitRepositoriesPlugin::all_entries(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("git-repositories"),
                priority: 28,
                title: String::from("󰘬 Git Repositories"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    fn all_entries() -> Vec<crate::model::Entry> {
        let git_repository_paths: Vec<String> =
            crate::plugin::utils::read_index_file("git-repositories-index.json");

        let home = std::env::var("HOME").unwrap_or(String::from(""));

        return git_repository_paths
            .into_iter()
            .filter_map(|git_repository_path| {
                let git_repository_display_name = git_repository_path.replacen(&home, "~", 1);

                return Some(crate::model::Entry {
                    id: git_repository_path,
                    title: git_repository_display_name,
                    action: String::from("focus"),
                    meta: String::from("windows"),
                });
            })
            .collect();
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
        let filtered_entries = crate::plugin::utils::search(self.all_entries.clone(), query);

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
        let terminal_launch_result = std::process::Command::new("alacritty")
            .arg("--working-directory")
            .arg(&entry_id)
            .spawn();

        if let Err(error) = terminal_launch_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to launch terminal",
            );
        }

        let editor_launch_result = std::process::Command::new("sublime_text")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn();

        if let Err(error) = editor_launch_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to launch editor",
            );
        }

        let git_ui_launch_result = std::process::Command::new("sublime_merge")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn();

        if let Err(error) = git_ui_launch_result {
            log::warn!(
                error = log::as_error!(error);
                "Failed to launch git ui",
            );
        }

        self.plugin_channel_out.try_send(crate::Message::Exit).ok();
    }
}
