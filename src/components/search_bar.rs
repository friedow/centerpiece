use gtk4::{prelude::*, Entry};

pub fn build() -> Entry {
    let search_bar = Entry::new();
    search_bar.set_placeholder_text(Some("Search or jump to..."));

    return search_bar;
}
