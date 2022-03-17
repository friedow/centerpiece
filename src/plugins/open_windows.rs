use glib::ObjectExt;
use gtk4::gdk::Key;
use gtk4::{Box, Widget};
use json::JsonValue;
use std::process::Command;
use subprocess::Exec;

use crate::components::list_item;
use crate::plugins::plugin::Plugin;

pub struct OpenWindowsPlugin();

impl Plugin for OpenWindowsPlugin {
    fn get_name(&self) -> String {
        return String::from("Open Windows");
    }

    fn build_items(&self) -> Vec<Box> {
        let i3_msg_output = Command::new("i3-msg")
            .arg("-t")
            .arg("get_tree")
            .output()
            .expect("failed to execute process")
            .stdout;

        let i3_msg_string = String::from_utf8_lossy(&i3_msg_output);
        let i3_msg_json = json::parse(&i3_msg_string).unwrap();

        let windows = find_windows(&i3_msg_json);

        return windows.into_iter().map(to_list_item).collect();
    }

    fn on_key_pressed(&self, list_item: &Widget, _key: Key) {
        unsafe {
            let window_id = list_item.data::<u32>("window_id").unwrap().as_ref();
            let focus_window_string = format!("[con_id={}] focus", window_id);
            let _result = Exec::cmd("i3-msg").arg(focus_window_string).join();
        }
    }
}

fn find_windows(i3_msg_part: &JsonValue) -> Vec<&JsonValue> {
    if !i3_msg_part["window_type"].is_null() {
        let mut windows = Vec::new();
        windows.push(i3_msg_part);
        return windows;
    } else if !i3_msg_part["nodes"].is_null() {
        return i3_msg_part["nodes"]
            .members()
            .flat_map(find_windows)
            .collect();
    } else {
        return Vec::new();
    }
}

fn to_list_item(window: &JsonValue) -> Box {
    let title = String::from(window["name"].as_str().unwrap());
    let window_id = window["id"].as_u32().unwrap();

    let action_text = String::from("switch to");

    let list_item = list_item::build(title, action_text);
    unsafe { list_item.set_data("window_id", window_id) }
    return list_item;
}
