#[derive(serde::Serialize)]
pub(crate) struct ItemGroup {
    pub name: String,
    pub icon: String,
    pub items: Vec<ListItem>,
}

#[derive(serde::Serialize)]
pub(crate) struct ListItem {
    pub title: String,
    pub actions: Vec<ListItemAction>,
}

#[derive(serde::Serialize)]
pub(crate) struct ListItemAction {
    pub keys: Vec<String>,
    pub text: String,
    pub command: ListItemActionCommand,
}

#[derive(serde::Serialize)]
pub(crate) struct ListItemActionCommand {
    pub program: String,
    pub args: Vec<String>,
}
