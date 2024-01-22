use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct GitRepositoriesPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for GitRepositoriesPlugin {
    fn id() -> &'static str {
        "git-repositories"
    }

    fn priority() -> u32 {
        28
    }

    fn title() -> &'static str {
        "󰘬 Git Repositories"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn new() -> Self {
        Self { entries: vec![] }
    }

    // This lint seems to be a false positive
    #[allow(clippy::unnecessary_filter_map)]
    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let git_repository_paths: Vec<String> =
            crate::plugin::utils::read_index_file("git-repositories-index.json");

        let home = std::env::var("HOME").unwrap_or(String::from(""));

        self.entries = git_repository_paths
            .into_iter()
            .filter_map(|git_repository_path| {
                let git_repository_display_name = git_repository_path.replacen(&home, "~", 1);

                Some(crate::model::Entry {
                    id: git_repository_path,
                    title: git_repository_display_name,
                    action: String::from("focus"),
                    meta: String::from("windows"),
                    command: None,
                })
            })
            .collect();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("alacritty")
            .arg("--working-directory")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch terminal while activating entry with id '{}'.",
                entry.id
            ))?;

        std::process::Command::new("sublime_text")
            .arg("--new-window")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch editor while activating entry with id '{}'.",
                entry.id
            ))?;

        std::process::Command::new("sublime_merge")
            .arg("--new-window")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch git ui while activating entry with id '{}'.",
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
