#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub priority: u32,
    pub title: String,
    pub entries: Vec<Entry>,
    pub app_channel_out: iced::futures::channel::mpsc::Sender<PluginRequest>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub action: String,
    pub meta: String,
}

pub enum PluginRequest {
    Search(String),
    Timeout,
}
