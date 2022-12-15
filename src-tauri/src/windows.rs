use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::types;

#[derive(Serialize, Deserialize)]
struct WindowTree {
    id: i64,
    name: Option<String>,
    pid: Option<u64>,
    nodes: Vec<WindowTree>,
}

impl WindowTree {
    fn to_list_item(&self) -> types::ListItem {
        return types::ListItem {
            title: self.name.to_owned().unwrap_or(String::from("null")),
            actions: vec![types::ListItemAction {
                keys: vec!["↵".into()],
                text: "switch to".into(),
                command: types::ListItemActionCommand {
                    program: String::from("sh"),
                    args: vec![
                        String::from("-c"),
                        format!("swaymsg [con_id={}] focus", self.id).into(),
                    ],
                },
            }],
        };
    }

    fn get_list_items(&self) -> Vec<types::ListItem> {
        if self.pid.is_some() && self.name.is_some() {
            return vec![self.to_list_item()];
        }
        return self
            .nodes
            .iter()
            .flat_map(|child_window_tree| child_window_tree.get_list_items())
            .collect();
    }
}

fn get_window_tree() -> WindowTree {
    let i3msg_command_output = Command::new("swaymsg")
        .arg("-t")
        .arg("get_tree")
        .output()
        .expect("failed to execute process");
    let window_tree_string = String::from_utf8_lossy(&i3msg_command_output.stdout).into_owned();
    let window_tree: WindowTree = serde_json::from_str(window_tree_string.as_str()).unwrap();
    return window_tree;
}

#[tauri::command]
pub(crate) fn get_windows_group() -> types::ItemGroup {
    let window_tree = get_window_tree();
    let mut list_items = window_tree.get_list_items();

    list_items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    list_items.dedup_by(|a, b| a.title.to_lowercase().eq(&b.title.to_lowercase()));

    return types::ItemGroup {
        name: "Windows".into(),
        icon: "LayoutGrid".into(),
        items: list_items,
    };
}
