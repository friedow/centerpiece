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
    open: String,
    command: Vec<String>,
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

fn to_list_item(desktop_file_path: String) -> ListItem {
    let desktop_file_contents = fs::read_to_string(&desktop_file_path).unwrap();
    println!("{desktop_file_contents}");

    lazy_static! {
      static ref NAME_REGEX: Regex = Regex::new(r"Name=(.*)").unwrap();
      static ref EXEC_REGEX: Regex = Regex::new(r"Exec=(.*)").unwrap();
    }

    let desktop_entry_name = NAME_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|name| name.as_str().to_string());
    }).unwrap_or("===No Title Found===".to_string());

    let desktop_entry_exec = EXEC_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|exec| exec.as_str().to_string());
    }).unwrap_or("".to_string());

    println!("{desktop_entry_exec}");

    return ListItem {
        title: desktop_entry_name,
        action: ListItemAction {
            keys: vec!["↵".into()],
            text: "Open".into(),
            open: desktop_entry_exec,
            command: Vec::new(),
        },
    };
}

#[tauri::command]
fn get_applications_group() -> ItemGroup {
    let desktop_file_paths = get_desktop_file_paths();
    let list_items: Vec<ListItem> = desktop_file_paths
        .into_iter()
        .map(to_list_item)
        .rev()
        .collect();

    return ItemGroup {
        name: "Apps".into(),
        icon: "rocket".into(),
        items: list_items,
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_applications_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
