// use walkdir::{DirEntry, WalkDir};

use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk4::glib;
use gtk4::prelude::*;

fn main() {
    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(build_ui);
    // Run the application
    app.run();
}

// When the application is launched…
fn build_ui(application: &gtk4::Application) {
    // … create a new window …
    let window = gtk4::ApplicationWindow::builder()
        .application(application)
        .title("Tucan Search")
        .default_height(500)
        .default_width(500)
        .modal(true)
        .build();

    let button = gtk4::Button::builder()
        .label("close")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand_set(true)
        .vexpand_set(true)
        .build();

    button.connect_clicked(clone!(@weak window => move |_| window.close()));

    window.set_child(Some(&button));

    window.present();
}

// fn main() {
//     list_git_directories();
// }

// fn is_dir(entry: &DirEntry) -> bool {
//     let is_dir = entry.file_type().is_dir();
//     let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
//     return is_dir && (!is_hidden || is_git_dir(entry));
// }

// fn is_git_dir(entry: &DirEntry) -> bool {
//     return entry.file_name().to_str().unwrap().eq(".git");
// }

// fn list_git_directories() {
//     let walker = WalkDir::new("/home/christian").into_iter();
//     for entry in walker
//         .filter_entry(|e| is_dir(e))
//         .filter_map(|e| e.ok())
//         .filter(|e| is_git_dir(e))
//     {
//         println!("{}", entry.path().display());
//     }
// }
