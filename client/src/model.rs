#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub priority: u32,
    pub title: String,
    pub entries: Vec<Entry>,
    pub app_channel_out: async_channel::Sender<PluginRequest>,
}

#[derive(Debug, Clone, Ord, PartialOrd)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub action: String,
    pub meta: String,
    pub command: Option<Vec<String>>,
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub enum PluginRequest {
    Search(String),
    Timeout,
    Activate(Entry),
}
