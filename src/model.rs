#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub priority: u32,
    pub title: String,
    pub entries: Vec<Entry>,
    pub app_channel_out: iced::futures::channel::mpsc::Sender<PluginRequest>,
}

#[derive(Debug, Clone, Eq)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub action: String,
    pub meta: String,
    pub cmd: Vec<String>,
}

impl std::hash::Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub enum PluginRequest {
    Search(String),
    Timeout,
    Activate(String),
}
