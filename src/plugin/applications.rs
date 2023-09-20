use fuzzy_matcher::FuzzyMatcher;
use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

pub struct ApplicationsPlugin {
    plugin: crate::model::Plugin,
    last_query: String,
    all_entries: std::collections::HashSet<DesktopEntry>,
    plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
}

#[derive(Debug)]
struct DesktopEntry {
    cmd: Vec<String>,
    entry: crate::model::Entry,
}

impl std::hash::Hash for DesktopEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entry.id.hash(state);
    }
}

impl Eq for DesktopEntry {}

impl PartialEq for DesktopEntry {
    fn eq(&self, other: &Self) -> bool {
        return self.entry.id == other.entry.id;
    }
}

impl DesktopEntry {
    pub fn try_from(path: &std::path::PathBuf) -> Result<DesktopEntry, ParsingError> {
        let bytes = std::fs::read_to_string(path)?;
        let desktop_entry = freedesktop_desktop_entry::DesktopEntry::decode(&path, &bytes)?;

        if !is_visible(&desktop_entry) {
            log::info!(appid = log::as_serde!(desktop_entry.appid); "Desktop entry is hidden");
            return Err(ParsingError::IsHidden);
        }

        let locale = std::env::var("LANG").unwrap_or(String::from("en_US"));
        let name_option = desktop_entry.name(Some(&locale));
        if name_option.is_none() {
            log::warn!(appid = log::as_serde!(desktop_entry.appid); "Unable to find name for desktop entry");
            return Err(ParsingError::MissingName);
        }

        // error on entries missing an exec command
        let exec_option = desktop_entry.exec();
        if exec_option.is_none() {
            log::warn!(appid = log::as_serde!(desktop_entry.appid); "Unable to find exec for desktop entry");
            return Err(ParsingError::MissingExec);
        }

        let cmd = exec_option.unwrap()
            .split_ascii_whitespace()
            .filter_map(|s| {
                if s.starts_with("%") {
                    None
                } else {
                    Some(String::from(s))
                }
            })
            .collect();

        let mut meta = desktop_entry
            .keywords()
            .unwrap_or(std::borrow::Cow::from(""))
            .replace(";", " ");
        meta.push_str(" Applications Apps");

        return Ok(DesktopEntry {
            cmd,
            entry: crate::model::Entry {
                id: desktop_entry.appid.to_string(),
                title: name_option.unwrap().to_string(),
                action: String::from("open"),
                meta,
            },
        });
    }
}

fn is_visible(desktop_entry: &freedesktop_desktop_entry::DesktopEntry) -> bool {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or(String::from("sway"));
    // filter entries where NotShowIn == current desktop
    if let Some(not_show_in) = desktop_entry.desktop_entry("NotShowIn") {
        let not_show_in_desktops = not_show_in.to_ascii_lowercase();

        if not_show_in_desktops.split(';').any(|d| d == desktop) {
            return false;
        }
    }

    // filter entries where OnlyShowIn != current desktop
    if let Some(only_show_in) = desktop_entry.only_show_in() {
        let only_show_in_desktops = only_show_in.to_ascii_lowercase();

        if !only_show_in_desktops.split(';').any(|d| d == desktop) {
            return false;
        }
    }

    // filter entries where NoDisplay != true
    if desktop_entry.no_display() {
        return false;
    }

    // filter entries where Exec == false
    if let Some(exec) = desktop_entry.exec() {
        if exec.to_ascii_lowercase() == "false" {
            return false;
        }
    }

    return true;
}

#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("unable to read desktop file")]
    ReadError(#[from] std::io::Error),
    #[error("unable to decode desktop file")]
    DecodeError(#[from] freedesktop_desktop_entry::DecodeError),
    #[error("desktop entry is hidden")]
    IsHidden,
    #[error("desktop entry is missing a name")]
    MissingName,
    #[error("desktop entry is missing an exec")]
    MissingExec,
}

impl ApplicationsPlugin {
    pub fn spawn() -> iced::Subscription<crate::Message> {
        return iced::subscription::channel(
            std::any::TypeId::of::<ApplicationsPlugin>(),
            100,
            |plugin_channel_out| async {
                let mut plugin = ApplicationsPlugin::new(plugin_channel_out);
                plugin.main().await
            },
        );
    }

    pub fn new(
        plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> ApplicationsPlugin {
        let (app_channel_out, plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

        return ApplicationsPlugin {
            last_query: String::new(),
            all_entries: std::collections::HashSet::<DesktopEntry>::new(),
            plugin_channel_in,
            plugin_channel_out,
            plugin: crate::model::Plugin {
                id: String::from("applications"),
                priority: 0,
                title: String::from("󰅐 Apps"),
                app_channel_out,
                entries: vec![],
            },
        };
    }

    async fn main(&mut self) -> ! {
        self.register_plugin().await;
        self.update_entries().await;

        loop {
            self.update().await;
        }
    }

    async fn register_plugin(&mut self) {
        let _ = self
            .plugin_channel_out
            .send(crate::Message::RegisterPlugin(self.plugin.clone()))
            .await;
    }

    async fn update(&mut self) {
        let plugin_request = self.plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(query).await,
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id),
        }
    }

    async fn update_entries(&mut self) {
        let paths = freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths());
        let desktop_entries: Vec<DesktopEntry> = paths.filter_map(|path| {
            let desktop_entry_result = DesktopEntry::try_from(&path);

            if let Err(error) = desktop_entry_result {
                log::warn!(err = log::as_error!(error); "Desktop entry cration failed");
                return None;
            }
            return Some(desktop_entry_result.unwrap());
        }).collect();
        self.all_entries = std::collections::HashSet::from_iter(desktop_entries.into_iter());

        self.search(self.last_query.clone()).await;
    }

    async fn search(&mut self, query: String) {
        self.last_query = query.clone();

        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

        let mut filtered_entries = self
            .all_entries
            .iter()
            .filter_map(|entry| {
                let keywords = std::format!("{} {}", entry.entry.title, entry.entry.meta);
                let match_result = matcher.fuzzy_indices(&keywords, &query);
                if match_result.is_none() {
                    return None;
                }
                let (score, _) = match_result.unwrap();
                return Some((score, entry));
            })
            .collect::<Vec<(i64, &DesktopEntry)>>();

        filtered_entries.sort_by_key(|(score, _)| score.clone());
        filtered_entries.reverse();

        // TODO: it may be more performant to convert this into a send_all
        let _ = self
            .plugin_channel_out
            .send(crate::Message::Clear(self.plugin.id.clone()))
            .await;

        for (_, entry) in filtered_entries {
            let _ = self
                .plugin_channel_out
                .send(crate::Message::AppendEntry(
                    self.plugin.id.clone(),
                    entry.entry.clone(),
                ))
                .await;
        }
    }

    fn activate(&mut self, entry_id: String) {
        let entry_option = self.all_entries.iter().find(|e| e.entry.id == entry_id);
        if entry_option.is_none() {
            log::warn!("Entry activation failed: Unable to find entry with id {}.", entry_id);
            return;
        }

        let cmd = entry_option.unwrap().cmd.clone();

        std::process::Command::new(cmd[0].clone())
            .args(&cmd[1..])
            .spawn()
            .expect("Command failure");

        self.plugin_channel_out.try_send(crate::Message::Exit).ok();
    }
}
