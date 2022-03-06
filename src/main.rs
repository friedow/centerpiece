use walkdir::{DirEntry, WalkDir};

use gtk4::prelude::*;

struct OmniboxOption {
    title: String,
    action_text: String,
}

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
        // .default_height(500)
        // .default_width(500)
        .modal(true)
        .build();

    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let search_bar = gtk4::SearchEntry::new();

    let option_list = build_option_list();

    vbox.append(&search_bar);
    vbox.append(&option_list);

    window.set_child(Some(&vbox));

    window.present();
}

fn build_option_list() -> gtk4::ScrolledWindow {
    let options: Vec<gtk4::Label> = list_git_directories()
        .into_iter()
        .map(|e| build_option(e))
        .collect();

    let option_list = gtk4::ListBox::new();
    for option in options {
        option_list.append(&option);
    }

    return gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(500)
        .height_request(500)
        .child(&option_list)
        .build();
}

fn build_option(label: String) -> gtk4::Label {
    return gtk4::Label::new(Some(&label));
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_dir(entry));
}

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn dir_name(entry: &DirEntry) -> String {
    return entry.path().display().to_string();
}

fn list_git_directories() -> Vec<String> {
    let walker = WalkDir::new("/home/christian").into_iter();
    let test = walker
        .filter_entry(|e| is_dir(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_git_dir(e))
        .map(|e| dir_name(&e));
    let toast = test.collect();

    return toast;
}
