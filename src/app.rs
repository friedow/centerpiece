use gtk4::{prelude::*, Application, ApplicationWindow, Box, Entry, Orientation};

use crate::components::list;

pub fn build(application: &Application) {
    let search_bar = Entry::new();
    search_bar.set_placeholder_text(Some("Search or jump to..."));

    let option_list = list::build();

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&search_bar);
    vbox.append(&option_list);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Tucan Search")
        .default_width(800)
        .modal(true)
        .child(&vbox)
        .build();

    window.present();
}
