#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use walkdir::{DirEntry, WalkDir};

fn is_desktop_file(entry: &DirEntry) -> bool {
  entry.file_name()
       .to_str()
       .map(|s| s.ends_with(".desktop"))
       .unwrap_or(false)
}

#[tauri::command]
fn get_desktop_files() -> Option<String> {

  let xdg_data_dirs = env!("XDG_DATA_DIRS").split(":");
  println!("{}", env!("XDG_DATA_DIRS"));
  for xdg_data_dir in xdg_data_dirs {
    println!("{}", xdg_data_dir);

    


    for dir_entry_promise in WalkDir::new(xdg_data_dir).follow_links(true) {
        let dir_entry_promise_ref = dir_entry_promise.as_ref();
        if let Err(e) = dir_entry_promise_ref {
          println!("error parsing direntry: {e:?}");
          continue;
        }
        let dir_entry = dir_entry_promise_ref.unwrap();

        if dir_entry.path().to_str()?.ends_with(".desktop") { 
          println!("{}", dir_entry.path().display());
         };
    }
  }

  return Some("Hello from Rust!".into());
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_desktop_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
