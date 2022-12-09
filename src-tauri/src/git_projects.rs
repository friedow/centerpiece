use lazy_static::lazy_static;
use regex::Regex;
use std::env::var;
use walkdir::{DirEntry, WalkDir};

use crate::types;

lazy_static! {
    static ref HOME: String = var("HOME").unwrap();
}

fn get_git_dirs() -> Vec<String> {
    let mut git_dirs = Vec::new();

    for dir_entry_promise in WalkDir::new(HOME.as_str()).follow_links(true) {
        // ignore files that return not found while accessing them
        if let Err(_) = dir_entry_promise {
            continue;
        };

        let dir_entry = dir_entry_promise.unwrap();

        // ignore files that return not found while accessing them
        let dir_entry_path_promise = dir_entry.path().to_str();
        if dir_entry_path_promise.is_none() {
            continue;
        }
        let dir_entry_path = dir_entry_path_promise.unwrap();

        // exclude config dirs
        let mut home_config_prefix = HOME.to_owned();
        home_config_prefix.push_str("/.");
        // let home_config_prefix = owned_home.push_str("/.");

        if dir_entry_path.starts_with(home_config_prefix.as_str()) {
            continue;
        }

        if dir_entry_path.ends_with("/.git") {
            println!("{}", dir_entry_path);
            git_dirs.push(dir_entry_path.to_string());
        };
    }

    return git_dirs;
}

fn get_project_name(git_dir: &String) -> Option<String> {
    lazy_static! {
        static ref PROJECT_NAME_REGEX: Regex = Regex::new(r"/([^/]+)/.git$").unwrap();
    }
    let desktop_entry_name_promise = PROJECT_NAME_REGEX.captures(git_dir).and_then(|cap| {
        return cap.get(1).map(|name| name.as_str().to_string());
    });

    if desktop_entry_name_promise.is_none() {
        return None;
    }
    return Some(desktop_entry_name_promise.unwrap());
}

fn to_list_item(git_dir: String) -> Option<types::ListItem> {
    let project_name_promise = get_project_name(&git_dir);

    if project_name_promise.is_none() {
        return None;
    }

    let project_name = project_name_promise.unwrap();
    let project_path = git_dir.replace("/.git", "");

    return Some(types::ListItem {
        title: project_name,
        actions: vec![types::ListItemAction {
            keys: vec!["↵".into()],
            text: "open".into(),
            command: types::ListItemActionCommand {
                program: String::from("sh"),
                args: vec![String::from("-c"), format!("code {}", project_path).into()],
            },
        }],
    });
}

pub(crate) fn get_git_projects_group() -> types::ItemGroup {
    let mut list_items: Vec<types::ListItem> = get_git_dirs()
        .into_iter()
        .map(to_list_item)
        .filter(|list_item_option| list_item_option.is_some())
        .map(|list_item_option| list_item_option.unwrap())
        .rev()
        .collect();

    list_items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    list_items.dedup_by(|a, b| a.title.to_lowercase().eq(&b.title.to_lowercase()));

    return types::ItemGroup {
        name: "Git Repositories".into(),
        icon: "folder".into(),
        items: list_items,
    };
}
