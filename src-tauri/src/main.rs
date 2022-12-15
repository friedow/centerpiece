mod applications;
mod git_projects;
mod types;
mod windows;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            applications::get_applications_group,
            windows::get_windows_group,
            git_projects::get_git_projects_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
