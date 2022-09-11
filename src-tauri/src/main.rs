#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct ItemGroup {
    name: String,
    icon: String,
    items: Vec<ListItem>,
}

#[derive(serde::Serialize)]
struct ListItem {
    title: String,
    action: ListItemAction,
}

#[derive(serde::Serialize)]
struct ListItemAction {
    keys: Vec<String>,
    text: String,
    open: Option<String>,
    command: Option<ListItemActionCommand>,
}

#[derive(serde::Serialize)]
struct ListItemActionCommand {
    program: String,
    args: Vec<String>,
}

use std::fs;
use walkdir::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;

fn get_desktop_file_paths() -> Vec<String> {
    let mut desktop_file_paths = Vec::new();

    let xdg_data_dirs = env!("XDG_DATA_DIRS").split(":");
    for xdg_data_dir in xdg_data_dirs {
        for dir_entry_promise in WalkDir::new(xdg_data_dir).follow_links(true) {
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

            if dir_entry_path.ends_with(".desktop") {
                desktop_file_paths.push(dir_entry_path.to_string())
            };
        }
    }

    return desktop_file_paths;
}

fn to_list_item(desktop_file_path: String) -> Option<ListItem> {
    let desktop_file_contents = fs::read_to_string(&desktop_file_path).unwrap();

    lazy_static! {
      static ref NAME_REGEX: Regex = Regex::new(r"\nName=(.*)").unwrap();
      static ref EXEC_REGEX: Regex = Regex::new(r"\nExec=(.*)").unwrap();
      static ref NO_DISPLAY_REGEX: Regex = Regex::new(r"\n(NoDisplay=true|Hidden=true)").unwrap();

      static ref TYPE_APPLICATION_REGEX: Regex = Regex::new(r"\n(Type=Application)").unwrap();
    }

    let desktop_entry_name_promise = NAME_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|name| name.as_str().to_string());
    });
    if desktop_entry_name_promise.is_none() { return None; }
    let desktop_entry_name = desktop_entry_name_promise.unwrap();

    let desktop_entry_exec_promise = EXEC_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|exec| exec.as_str().to_string());
    });
    if desktop_entry_exec_promise.is_none() { return None; }

    let desktop_entry_no_display_promise = NO_DISPLAY_REGEX.captures(&desktop_file_contents).and_then(|captures| {
        return captures.get(1).map(|capture| capture.as_str().to_string());
    });
    if desktop_entry_no_display_promise.is_some() { return None; }

    let desktop_entry_type_application_promise = TYPE_APPLICATION_REGEX.captures(&desktop_file_contents).and_then(|captures| {
        return captures.get(1).map(|capture| capture.as_str().to_string());
    });
    if desktop_entry_type_application_promise.is_none() { return None; }

    return Some(ListItem {
        title: desktop_entry_name,
        action: ListItemAction {
            keys: vec!["↵".into()],
            text: "open".into(),
            open: Some(desktop_file_path),
            command: None,
        },
    });
}

#[tauri::command]
fn get_applications_group() -> ItemGroup {
    let desktop_file_paths = get_desktop_file_paths();
    let mut list_items: Vec<ListItem> = desktop_file_paths
        .into_iter()
        .map(to_list_item)
        .filter(|list_item_option| list_item_option.is_some())
        .map(|list_item_option| list_item_option.unwrap())
        .rev()
        .collect();
    
    list_items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    list_items.dedup_by(|a, b| a.title.to_lowercase().eq(&b.title.to_lowercase()));

    return ItemGroup {
        name: "Apps".into(),
        icon: "rocket".into(),
        items: list_items,
    };
}

use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WindowTree {
    id: i64,
    name: Option<String>,
    window_type: Option<String>,
    nodes: Vec<WindowTree>,
}

impl WindowTree {
    fn to_list_item(&self) -> ListItem {
        return ListItem {
            title: self.name.to_owned().unwrap_or(String::from("null")),
            action: ListItemAction {
                keys: vec!["↵".into()],
                text: "switch to".into(),
                open: None,
                command: Some(ListItemActionCommand {
                    program: String::from("i3-msg"),
                    args: vec![format!("[con_id={}]", self.id).into(), String::from("focus")]
                }),
            },
        }
    }


    fn get_list_items(&self) -> Vec<ListItem> {
        if self.window_type.is_some() && self.name.is_some() {
            return vec![self.to_list_item()];
        }
        return self.nodes.iter().flat_map(|child_window_tree| child_window_tree.get_list_items()).collect();
    }
}

fn get_window_tree() -> WindowTree {
        let i3msg_command_output = Command::new("i3-msg")
            .arg("-t")
            .arg("get_tree")
            .output()
            .expect("failed to execute process");
        let window_tree_string = String::from_utf8_lossy(&i3msg_command_output.stdout).into_owned();
        let window_tree: WindowTree = serde_json::from_str(window_tree_string.as_str()).unwrap();
        return window_tree;
}

#[tauri::command]
fn get_windows_group() -> ItemGroup {
    let window_tree = get_window_tree();
    let mut list_items = window_tree.get_list_items();
    
    list_items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    list_items.dedup_by(|a, b| a.title.to_lowercase().eq(&b.title.to_lowercase()));

    return ItemGroup {
        name: "Windows".into(),
        icon: "window-maximize".into(),
        items: list_items,
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_applications_group, get_windows_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
