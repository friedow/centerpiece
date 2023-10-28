use anyhow::Context;
use iced::futures::StreamExt;

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

    fn log_and_panic(error: anyhow::Error) {
        log::error!(
            target: Self::id(),
            "{:?}", error,
        );
        panic!();
    }

    async fn main(
        &mut self,
        mut plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
        mut app_channel_out: iced::futures::channel::mpsc::Sender<crate::model::PluginRequest>,
        mut plugin_channel_in: iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>,
    ) -> anyhow::Result<()> {
        self.register_plugin(&mut plugin_channel_out, &mut app_channel_out)?;

        loop {
            self.update(&mut plugin_channel_out, &mut plugin_channel_in).await?;
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

    async fn update(&mut self, plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>, plugin_channel_in: &mut iced::futures::channel::mpsc::Receiver<crate::model::PluginRequest>) -> anyhow::Result<()> {
        let plugin_request = plugin_channel_in.select_next_some().await;

        match plugin_request {
            crate::model::PluginRequest::Search(query) => self.search(&query, plugin_channel_out)?,
            crate::model::PluginRequest::Timeout => (),
            crate::model::PluginRequest::Activate(entry_id) => self.activate(entry_id, plugin_channel_out)?,
        }

        return Ok(());
    }

    fn search(&mut self, query: &String, plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>) -> anyhow::Result<()> {
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

    fn activate(&mut self, entry_id: String, plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>) -> anyhow::Result<()>;
}
