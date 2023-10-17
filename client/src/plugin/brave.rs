use std::vec;

use anyhow::Context;
use iced::futures::StreamExt;

pub struct BravePlugin {
    pwa_plugin: crate::model::Plugin,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

#[derive(serde::Deserialize)]
struct BookmarksFile {
    roots: BookmarksRoot,
}

#[derive(serde::Deserialize)]
struct BookmarksRoot {
    bookmark_bar: Bookmark,
    other: Bookmark,
    synced: Bookmark,
}
impl Into<Bookmark> for BookmarksRoot {
    fn into(self) -> Bookmark {
        return Bookmark::Folder(FolderBookark {
            name: String::from("roots"),
            children: vec![self.bookmark_bar, self.other, self.synced],
        });
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
enum Bookmark {
    Folder(FolderBookark),
    Url(UrlBookmark),
}

#[derive(serde::Deserialize, Debug, Clone)]
struct FolderBookark {
    name: String,
    children: Vec<Bookmark>,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct UrlBookmark {
    name: String,
    url: String,
}

impl Into<crate::model::Entry> for &UrlBookmark {
    fn into(self) -> crate::model::Entry {
        return crate::model::Entry {
            id: self.url.clone(),
            title: self.name.clone(),
            action: String::from("open"),
            meta: String::from("Bookmarks"),
        };
    }
}

impl BravePlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<BravePlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = BravePlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> BravePlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        let pwa_entries_result = BravePlugin::all_pwa_entries();
        if let Err(error) = pwa_entries_result {
            log::error!(
                target: "brave-progressive-web-apps",
                "{}", error
            );
            std::process::exit(1);
        }

        return BravePlugin {
            plugin_channel_in,
            plugin_channel_out,
            pwa_plugin: crate::model::Plugin {
                id: String::from("brave-progressive-web-apps"),
                priority: 28,
                title: String::from("󰀻 Progressive Web Apps"),
                app_channel_out,
                entries: pwa_entries_result.unwrap(),
            },
        };
    }

    fn all_pwa_entries() -> anyhow::Result<Vec<crate::model::Entry>> {
        let home_directory =
            std::env::var("HOME").context("Could not read HOME environment variable.")?;

        let index_file_path = std::path::Path::new(&home_directory)
            .join(".config/BraveSoftware/Brave-Browser/Default/Bookmarks");

        let bookmarks_file = std::fs::File::open(index_file_path)
            .context("Error while opening brave bookmarks file.")?;
        let reader = std::io::BufReader::new(bookmarks_file);
        let bookmarks_file_content: BookmarksFile =
            serde_json::from_reader(reader).context("Error while reading brave bookmarks file.")?;

        let bookmarks_root: Bookmark = bookmarks_file_content.roots.into();

        let pwa_folder = BravePlugin::find_bookmarks_folder_recursive(
            &bookmarks_root,
            &String::from("Progressive Web Apps"),
        )
        .ok_or(anyhow::anyhow!(
            "Unable to find a bookmarks folder named 'Progressive Web Apps'."
        ))?;

        let pwa_bookmarks = BravePlugin::get_bookmarks_recursive(pwa_folder)
            .into_iter()
            .map(|bookmark| bookmark.into())
            .collect();

        return Ok(pwa_bookmarks);
    }

    fn find_bookmarks_folder_recursive<'a>(
        bookmark: &'a Bookmark,
        folder_name: &String,
    ) -> Option<&'a Bookmark> {
        match bookmark {
            Bookmark::Folder(folder) => {
                if &folder.name == folder_name {
                    return Some(&bookmark);
                }

                for child in folder.children.iter() {
                    let find_bookmarks_option =
                        BravePlugin::find_bookmarks_folder_recursive(&child, folder_name);
                    if find_bookmarks_option.is_some() {
                        return find_bookmarks_option;
                    }
                }
                return None;
            }

            Bookmark::Url(_bookmark) => return None,
        };
    }

    fn get_bookmarks_recursive(bookmark: &Bookmark) -> Vec<&UrlBookmark> {
        return match bookmark {
            Bookmark::Folder(folder) => folder
                .children
                .iter()
                .flat_map(|b| BravePlugin::get_bookmarks_recursive(b))
                .collect(),

            Bookmark::Url(bookmark) => vec![bookmark],
        };
    }

    async fn main(&mut self) -> ! {
        let register_plugin_result = self.register_plugin();
        if let Err(error) = register_plugin_result {
            log::error!(
                target: self.pwa_plugin.id.as_str(),
                "{}", error,
            );
            std::process::exit(1);
        }

        loop {
            let update_result = self.update().await;
            if let Err(error) = update_result {
                log::warn!(
                    target: self.pwa_plugin.id.as_str(),
                    "{}", error,
                );
            }
        }
    }

    fn register_plugin(&mut self) -> anyhow::Result<()> {
        self.plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.pwa_plugin.clone()))
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
        let filtered_entries = crate::plugin::utils::search(self.pwa_plugin.entries.clone(), query);

        self.plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                self.pwa_plugin.id.clone(),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries while searching for '{}'.",
                query
            ))?;

        return Ok(());
    }

    fn activate(&mut self, entry_id: String) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(format!("--app={}", entry_id))
            .spawn()
            .context(format!(
                "Failed to launch brave in app mode while activating entry with id '{}'.",
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
