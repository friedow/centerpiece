use gtk4::{prelude::*, CenterBox, ListBox, PolicyType, ScrolledWindow};

use crate::components::list_item;
use crate::components::list_item_header;
use crate::stores::git_repositories;

pub fn build() -> ScrolledWindow {
    let options: Vec<CenterBox> = git_repositories::repositories()
        .into_iter()
        .map(list_item::build)
        .collect();

    let option_list = ListBox::new();
    option_list.set_margin_end(8);
    option_list.set_margin_start(8);
    option_list.set_header_func(list_item_header::update_header);
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
