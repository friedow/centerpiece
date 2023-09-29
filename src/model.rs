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

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, ,{})",
            self.id, self.title, self.action, self.meta
        )
    }
}

impl AsRef<str> for Entry {
    fn as_ref(&self) -> &str {
        self.title.as_str()
    }
}

pub enum PluginRequest {
    Search(String),
    Timeout,
    Activate(String),
}
