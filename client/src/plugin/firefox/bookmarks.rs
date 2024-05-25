use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct BookmarksPlugin {
    entries: Vec<crate::model::Entry>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)]
enum Section {
    #[serde(rename_all = "PascalCase")]
    Profile {
        name: String,
        is_relative: String,
        path: String,
        default: Option<String>,
    },
    #[serde(rename_all = "PascalCase")]
    General {
        start_with_last_profile: String,
        version: Option<String>,
    },

    #[serde(rename_all = "PascalCase")]
    Install { default: String, locked: String },
}

fn profile_path() -> anyhow::Result<String> {
    let home_directory = std::env::var("HOME")?;

    let profiles_file_path = format!("{home_directory}/.mozilla/firefox/profiles.ini");
    let profiles_file = std::fs::File::open(profiles_file_path)?;
    let profiles_file_contents: std::collections::HashMap<String, Section> =
        serde_ini::from_read(profiles_file)?;

    let mut default_profile = profiles_file_contents
        .values()
        .find(|section| match section {
            Section::Profile { default, .. } => {
                default.clone().unwrap_or(String::from("")) == String::from("1")
            }
            _ => false,
        });

    if default_profile.is_none() {
        default_profile = profiles_file_contents
            .values()
            .find(|section| match section {
                Section::Profile { .. } => true,
                _ => false,
            });
    }

    if default_profile.is_none() {
        return Err(anyhow::anyhow!("Could not find a firefox profile."));
    }

    match default_profile.unwrap() {
        Section::Profile {
            is_relative, path, ..
        } => {
            if is_relative.eq(&String::from("1")) {
                Ok(format!("{home_directory}/.mozilla/firefox/{path}"))
            } else {
                Ok(path.clone())
            }
        }
        _ => {
            unreachable!("A non-profile section should be parsed as a profile.");
        }
    }
}

impl Plugin for BookmarksPlugin {
    fn id() -> &'static str {
        "firefox_bookmarks"
    }

    fn priority() -> u32 {
        26
    }

    fn title() -> &'static str {
        "󰃃 Bookmarks"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        let profile_path = profile_path()?;

        println!("{:#?}", profile_path);

        let bookmarks_file_path = format!("{profile_path}/places.sqlite");
        let cache_directory = crate::plugin::utils::centerpiece_cache_directory()?;
        let bookmarks_cache_file_path = format!("{cache_directory}/fireforx-bookmarks.sqlite");

        std::fs::copy(bookmarks_file_path, &bookmarks_cache_file_path)
            .context("Error while creating cache directory")?;

        let connection = sqlite::open(bookmarks_cache_file_path)?;
        let query = "
            SELECT moz_bookmarks.title, moz_places.url
            FROM 
        	    moz_bookmarks
            	LEFT JOIN moz_places
                ON moz_bookmarks.fk = moz_places.id
            WHERE moz_bookmarks.type = 1
            ORDER BY moz_places.visit_count DESC";

        connection.execute(query)?;
        let url_rows = connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        self.entries = url_rows
            .map(|row| {
                let title = row.read::<&str, _>("title");
                let url = row.read::<&str, _>("url");

                crate::model::Entry {
                    id: url.to_string(),
                    title: title.to_string(),
                    action: String::from("open"),
                    meta: String::from("Bookmarks"),
                    command: None,
                }
            })
            .collect();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("firefox")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch firefox while activating entry with id '{}'.",
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
