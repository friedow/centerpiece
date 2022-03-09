use gtk4::{prelude::*, Box, Label, ListBoxRow, Orientation};

use crate::components::list_item;
use crate::structs::list_item_data::ListItemData;

pub fn update_header(row: &ListBoxRow, before: Option<&ListBoxRow>) {
    let row_data = list_item::get_row_data(row);
    if before.is_none() || list_item::get_row_data(before.unwrap()).group != row_data.group {
        let group_header = build(row_data);
        row.set_header(Some(&group_header));
        return;
    }
}

fn build(option_data: &ListItemData) -> Box {
    let header_label = Label::new(Some(&option_data.group));

    let vbox = Box::new(Orientation::Horizontal, 0);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);
    vbox.append(&header_label);

    return vbox;
}
