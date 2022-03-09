use walkdir::{DirEntry, WalkDir};

use crate::structs::list_item_data::ListItemData;

pub fn repositories() -> Vec<ListItemData> {
    let home = std::env::var("HOME").unwrap();
    return WalkDir::new(home)
        .into_iter()
        .filter_entry(is_dir)
        .filter_map(|e| e.ok())
        .filter(is_git_repository)
        .map(to_omnibox_option)
        .collect();
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_repository(entry));
}

fn is_git_repository(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn to_omnibox_option(entry: DirEntry) -> ListItemData {
    let home = std::env::var("HOME").unwrap();
    return ListItemData {
        group: String::from("Git Repositories"),
        title: entry
            .path()
            .display()
            .to_string()
            .replace("/.git", "")
            .replace(&home, "~"),
        action_text: String::from("open"),
    };
}
