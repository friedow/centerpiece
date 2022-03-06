use glib::Quark;
use walkdir::{DirEntry, WalkDir};

use gtk4::{prelude::*, CenterBox, ListBoxRow, Orientation};

struct OmniboxOption {
    group: String,
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
        .default_width(800)
        .modal(true)
        .build();

    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let search_bar = gtk4::Entry::new();
    search_bar.set_placeholder_text(Some("Search or jump to..."));

    let option_list = build_option_list();

    vbox.append(&search_bar);
    vbox.append(&option_list);

    window.set_child(Some(&vbox));

    window.present();
}

fn build_option_list() -> gtk4::ScrolledWindow {
    let options: Vec<gtk4::CenterBox> = list_git_directories()
        .into_iter()
        .map(|e| build_option(e))
        .collect();

    let option_list = gtk4::ListBox::new();
    option_list.set_margin_end(8);
    option_list.set_margin_start(8);
    option_list.set_header_func(|row, before| build_header(row, before));
    for option in options.into_iter() {
        option_list.append(&option);
    }

    return gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(500)
        .height_request(500)
        .child(&option_list)
        .build();
}

fn build_header(row: &ListBoxRow, before: Option<&ListBoxRow>) {
    if (before.is_some()) {
        let row_before = before.unwrap();
        let option_before = row_before.child().unwrap();
        unsafe {
            let omnibox_option = option_before
                .data::<OmniboxOption>("omnibox_option")
                .unwrap()
                .as_ref();

            let header_label = gtk4::Label::new(Some(&omnibox_option.group));

            let vbox = gtk4::Box::new(Orientation::Horizontal, 0);
            vbox.append(&header_label);

            row.set_header(Some(&vbox));
        }
    }
    // let header_label = gtk4::Label::new(Some("test"));

    // let vbox = gtk4::Box::new(Orientation::Horizontal, 0);
    // vbox.append(&header_label);

    // row.set_header(Some(&vbox))
}

fn build_option(option_data: OmniboxOption) -> gtk4::CenterBox {
    let title_label = gtk4::Label::new(Some(&option_data.title));
    let action_label = gtk4::Label::new(Some(&option_data.action_text));
    let hbox = gtk4::CenterBox::new();
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_end(8);
    hbox.set_margin_start(8);
    hbox.set_start_widget(Some(&title_label));
    hbox.set_end_widget(Some(&action_label));

    unsafe {
        hbox.set_data("omnibox_option", option_data);
    }
    return hbox;
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_dir(entry));
}

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn to_omnibox_option(entry: &DirEntry) -> OmniboxOption {
    return OmniboxOption {
        group: String::from("Git Projects"),
        title: entry.path().display().to_string(),
        action_text: String::from("open"),
    };
}

fn list_git_directories() -> Vec<OmniboxOption> {
    return WalkDir::new("/home/christian")
        .into_iter()
        .filter_entry(|e| is_dir(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_git_dir(e))
        .map(|e| to_omnibox_option(&e))
        .collect();
}
