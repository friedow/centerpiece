use walkdir::{DirEntry, WalkDir};

// use glib::clone;
// // glib and other dependencies are re-exported by the gtk crate
// use gtk::glib;
// use gtk::prelude::*;

// // When the application is launched…
// fn on_activate(application: &gtk::Application) {
//     // … create a new window …
//     let window = gtk::ApplicationWindow::new(application);
//     // … with a button in it …
//     let button = gtk::Button::with_label("Hello World!");
//     // … which closes the window when clicked
//     button.connect_clicked(clone!(@weak window => move |_| window.close()));
//     window.set_child(Some(&button));
//     window.present();
// }

// fn main() {
//     // Create a new application with the builder pattern
//     let app = gtk::Application::builder()
//         .application_id("com.github.gtk-rs.examples.basic")
//         .build();
//     app.connect_activate(on_activate);
//     // Run the application
//     app.run();
// }

fn main() {
    listGitDirectories();
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_dir(entry));
}

fn is_git_dir(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn listGitDirectories() {
    let walker = WalkDir::new("/home/christian").into_iter();
    for entry in walker
        .filter_entry(|e| is_dir(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_git_dir(e))
    {
        println!("{}", entry.path().display());
    }
}
