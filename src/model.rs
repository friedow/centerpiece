#[derive(Debug, Clone)]
pub struct PluginModel {
    pub id: String,
    pub priority: u32,
    pub title: String,
    pub entries: Vec<EntryModel>,
    pub app_channel_out: iced::futures::channel::mpsc::Sender<crate::plugin::PluginRequest>,
}

#[derive(Debug, Clone)]
pub struct EntryModel {
    pub id: String,
    pub title: String,
    pub action: String,
}
