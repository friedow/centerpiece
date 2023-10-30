use fuzzy_matcher::FuzzyMatcher;
use anyhow::Context;
use iced::futures::StreamExt;

pub fn spawn<PluginType: Plugin + std::marker::Send + 'static>(
) -> iced::Subscription<crate::Message> {
    return iced::subscription::channel(
        std::any::TypeId::of::<PluginType>(),
        100,
        |plugin_channel_out| async move {
            let mut plugin = PluginType::new();

            let main_loop_result = plugin.main(plugin_channel_out).await;
            if let Err(error) = main_loop_result {
                log::error!(
                    target: PluginType::id(),
                    "{:?}", error,
                );
                panic!();
            }

            loop {}
        },
    );
}

#[async_trait::async_trait]
pub trait Plugin {
    fn id() -> &'static str;
    fn priority() -> u32;
    fn title() -> &'static str;

    fn new() -> Self;

    fn entries(&self) -> Vec<crate::model::Entry>;

    fn plugin(
        &self,
        app_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
    ) -> crate::model::Plugin {
        return crate::model::Plugin {
            id: String::from(Self::id()),
            priority: Self::priority(),
            title: String::from(Self::title()),
            app_channel_out: app_channel_out.clone(),
            entries: self.entries(),
        };
    }

    async fn main(
        &mut self,
        mut plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let (mut app_channel_out, mut plugin_channel_in) =
            iced::futures::channel::mpsc::channel(100);
        self.register_plugin(&mut plugin_channel_out, &mut app_channel_out)?;

        loop {
            self.update(&mut plugin_channel_out, &mut plugin_channel_in)
                .await?;
        }
    }

    fn register_plugin(
        &mut self,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        app_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
    ) -> anyhow::Result<()> {
        plugin_channel_out
            .try_send(crate::Message::RegisterPlugin(self.plugin(app_channel_out)))
            .context("Failed to send message to register plugin.")?;

        return Ok(());
    }

    async fn update(
        &mut self,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
        plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
    ) -> anyhow::Result<()> {
        let plugin_request = plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => {
                self.search(&query, plugin_channel_out)?
            }
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => {
                self.activate(entry_id, plugin_channel_out)?
            }
        }

        return Ok(());
    }

    fn search(
        &mut self,
        query: &String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let filtered_entries = crate::plugin::utils::search(self.entries(), query);

        plugin_channel_out
            .try_send(crate::Message::UpdateEntries(
                String::from(Self::id()),
                filtered_entries,
            ))
            .context(format!(
                "Failed to send message to update entries while searching for '{}'.",
                query
            ))?;

        return Ok(());
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()>;
}


pub fn search(entries: Vec<crate::model::Entry>, query: &String) -> Vec<crate::model::Entry> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    if query.is_empty() {
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|entry| entry.title.clone());
        return sorted_entries;
    }

    let mut filtered_entries = entries
        .into_iter()
        .filter_map(|entry| {
            let keywords = format!("{} {}", entry.title, entry.meta);
            let match_result = matcher.fuzzy_indices(&keywords, query);
            if match_result.is_none() {
                return None;
            }
            let (score, _) = match_result.unwrap();
            return Some((score, entry));
        })
        .collect::<Vec<(i64, crate::model::Entry)>>();

    filtered_entries.sort_by(|(a_score, _), (b_score, _)| b_score.cmp(a_score));
    return filtered_entries
        .into_iter()
        .map(|(_, entry)| entry)
        .collect::<Vec<crate::model::Entry>>();
}

// TODO: this function should return a result and propagate errors
pub fn read_index_file<T>(file_name: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let home_directory_result = std::env::var("HOME");
    if let Err(error) = home_directory_result {
        log::error!(
            error = log::as_error!(error);
            "Could not read HOME environment variable",
        );
        panic!();
    }
    let home_directory = home_directory_result.unwrap();

    let index_file_path = std::path::Path::new(&home_directory)
        .join(".cache/centerpiece")
        .join(file_name);
    let index_file_result = std::fs::File::open(index_file_path);
    if let Err(error) = index_file_result {
        log::error!(
            error = log::as_error!(error);
            "Error while opening index file",
        );
        panic!();
    }
    let index_file = index_file_result.unwrap();

    let reader = std::io::BufReader::new(index_file);
    let git_repository_paths_result: Result<T, _> = serde_json::from_reader(reader);
    if let Err(error) = git_repository_paths_result {
        log::error!(
            error = log::as_error!(error);
            "Error while reading index file",
        );
        panic!();
    }
    let git_repository_paths = git_repository_paths_result.unwrap();
    git_repository_paths
}
