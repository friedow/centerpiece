use gtk4::{prelude::*, CenterBox, Label};

use crate::structs::list_item_data::ListItemData;

pub fn build(data: ListItemData) -> CenterBox {
    let title_label = Label::new(Some(&data.title));
    let action_label = Label::new(Some(&data.action_text));

    let hbox = CenterBox::new();
    hbox.set_start_widget(Some(&title_label));
    hbox.set_end_widget(Some(&action_label));
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_end(8);
    hbox.set_margin_start(8);
    unsafe {
        hbox.set_data("omnibox_option", data);
    }

    return hbox;
}
