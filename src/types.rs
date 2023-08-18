pub struct Plugin {
    pub id: String,
    pub title: String,
    pub entries: Vec<Entry>,
}

pub struct Entry {
    pub id: String,
    pub title: String,
    pub action: String,
}
