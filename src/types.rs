pub struct Plugin {
    pub title: String,
    pub entries: Vec<PluginEntry>,
}

pub struct PluginEntry {
    pub title: String,
    pub action: String,
}
