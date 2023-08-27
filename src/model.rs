#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub priority: u32,
    pub title: String,
    pub entries: Vec<Entry>,
    // pub channel: iced::futures::channel::mpsc::Sender<crate::plugin::clock::PluginRequest>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub action: String,
}
