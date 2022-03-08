use gtk4::{
    prelude::*, Box, CenterBox, Label, ListBox, ListBoxRow, Orientation, PolicyType, ScrolledWindow,
};
use walkdir::{DirEntry, WalkDir};

use crate::components::list_item;
use crate::structs::list_item_data::ListItemData;

pub fn build() -> ScrolledWindow {
    let options: Vec<CenterBox> = list_git_directories()
        .into_iter()
        .map(|e| list_item::build(e))
        .collect();

    let option_list = ListBox::new();
    option_list.set_margin_end(8);
    option_list.set_margin_start(8);
    option_list.set_header_func(|row, before| set_group_header(row, before));
    for option in options.into_iter() {
        option_list.append(&option);
    }

    return ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(500)
        .height_request(500)
        .child(&option_list)
        .build();
}

fn set_group_header(row: &ListBoxRow, before: Option<&ListBoxRow>) {
    let row_data = get_row_data(row);
    if before.is_none() || get_row_data(before.unwrap()).group != row_data.group {
        let group_header = build_group_header(row_data);
        row.set_header(Some(&group_header));
        return;
    }
}

fn build_group_header(option_data: &ListItemData) -> Box {
    let header_label = Label::new(Some(&option_data.group));
    let vbox = Box::new(Orientation::Horizontal, 0);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);
    vbox.append(&header_label);
    return vbox;
}

fn get_row_data(row: &ListBoxRow) -> &ListItemData {
    unsafe {
        let option = row.child().unwrap();
        return option
            .data::<ListItemData>("omnibox_option")
            .unwrap()
            .as_ref();
    }
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_dir(entry));
}

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn to_omnibox_option(entry: &DirEntry) -> ListItemData {
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

fn list_git_directories() -> Vec<ListItemData> {
    let home = std::env::var("HOME").unwrap();
    return WalkDir::new(home)
        .into_iter()
        .filter_entry(|e| is_dir(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_git_dir(e))
        .map(|e| to_omnibox_option(&e))
        .collect();
}
