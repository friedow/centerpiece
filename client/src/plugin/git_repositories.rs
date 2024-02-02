use crate::{plugin::utils::Plugin, settings::Settings};
use anyhow::Context;

pub struct GitRepositoriesPlugin {
    entries: Vec<crate::model::Entry>,
    settings: Settings,
}

impl Plugin for GitRepositoriesPlugin {
    fn id() -> &'static str {
        "git_repositories"
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
        let settings_result = Settings::new();
        if let Err(error) = settings_result {
            log::error!(target: Self::id(), "{:?}", error);
            panic!();
        }
        Self {
            entries: vec![],
            settings: settings_result.unwrap(),
        }
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
        let editor_command: Vec<String> = self
            .settings
            .plugin
            .git_repositories
            .editor_command
            .clone()
            .into_iter()
            .map(|command_part| {
                if command_part == "$GIT_DIRECTORY" {
                    entry.id.clone()
                } else {
                    command_part
                }
            })
            .collect();
        std::process::Command::new(&editor_command[0])
            .args(&editor_command[1..])
            .spawn()?;

        let git_ui_command: Vec<String> = self
            .settings
            .plugin
            .git_repositories
            .git_ui_command
            .clone()
            .into_iter()
            .map(|command_part| {
                if command_part == "$GIT_DIRECTORY" {
                    entry.id.clone()
                } else {
                    command_part
                }
            })
            .collect();
        std::process::Command::new(&git_ui_command[0])
            .args(&git_ui_command[1..])
            .spawn()?;

        let terminal_command: Vec<String> = self
            .settings
            .plugin
            .git_repositories
            .terminal_command
            .clone()
            .into_iter()
            .map(|command_part| {
                if command_part == "$GIT_DIRECTORY" {
                    entry.id.clone()
                } else {
                    command_part
                }
            })
            .collect();
        std::process::Command::new(&terminal_command[0])
            .args(&terminal_command[1..])
            .spawn()?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
