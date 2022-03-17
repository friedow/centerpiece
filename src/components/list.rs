use gtk4::gdk::{Key, ModifierType};
use gtk4::{prelude::*, Box, Label, ListBox, ListBoxRow, Orientation, PolicyType, ScrolledWindow};
use gtk4::{EventControllerKey, Inhibit};

use crate::plugins;

pub fn build() -> ScrolledWindow {
    let option_list = ListBox::new();
    option_list.set_margin_end(8);
    option_list.set_margin_start(8);
    option_list.set_header_func(set_header);
    add_key_event_controller(&option_list);

    for plugin in plugins::list().into_iter() {
        for option in plugin.as_ref().build_items().into_iter() {
            set_plugin_name(&option, plugin.as_ref().get_name());
            option_list.append(&option);
        }
    }

    return ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(500)
        .height_request(500)
        .child(&option_list)
        .build();
}

fn add_key_event_controller(hbox_option: &ListBox) {
    let list_box = hbox_option.clone();
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_controller, key, _keycode, modifier| {
        on_key_pressed(&list_box, key, modifier);
        return Inhibit(false);
    });
    hbox_option.add_controller(&controller)
}

fn on_key_pressed(option_list: &ListBox, key: Key, _modifier: ModifierType) {
    let key_name = key.name().unwrap();
    if key_name == "Up" || key_name == "Down" {
        return;
    }

    let option_row = option_list.selected_row().unwrap();
    let plugin_name = get_plugin_name(&option_row);

    for plugin in plugins::list().into_iter() {
        if plugin.as_ref().get_name().as_str() == plugin_name.as_str() {
            let option = option_list.selected_row().unwrap().child().unwrap();
            plugin.on_key_pressed(&option, key)
        }
    }
}

// plugin name handlers

fn get_plugin_name(option_row: &ListBoxRow) -> &String {
    unsafe {
        let option = option_row.child().unwrap();
        return option.data::<String>("plugin_name").unwrap().as_ref();
    }
}

fn set_plugin_name(option: &Box, plugin_name: String) {
    unsafe {
        option.set_data("plugin_name", plugin_name);
    }
}

// group headers

pub fn set_header(row: &ListBoxRow, before: Option<&ListBoxRow>) {
    let current_row_plugin_name = get_plugin_name(row);

    if before.is_none() || get_plugin_name(before.unwrap()) != current_row_plugin_name {
        let group_header = build_header(current_row_plugin_name);
        row.set_header(Some(&group_header));
        return;
    }
}

fn build_header(plugin_name: &String) -> Box {
    let header_label = Label::new(Some(plugin_name));

    let vbox = Box::new(Orientation::Horizontal, 0);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);
    vbox.append(&header_label);

    return vbox;
}

// fn on_row_activated(_list_box: &ListBox, list_box_row: &ListBoxRow) {
//     let list_item_data = list_item::get_data(list_box_row);
//     if list_item_data.group == String::from("Git Repositories") {
//         git_repositories::on_activate_list_item(list_item_data);
//     }
// }
