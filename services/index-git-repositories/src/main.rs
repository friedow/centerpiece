use rust_search::FilterExt;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let git_directory_paths: Vec<String> = rust_search::SearchBuilder::default()
        .location("~/")
        .search_input("\\.git")
        .limit(100)
        .strict()
        .hidden()
        .custom_filter(|dir| {
            let name = dir.file_name().to_str().unwrap();
            if name == ".git" {
                return true;
            }
            !name.starts_with('.')
        })
        .build()
        .collect();

    let git_repository_paths: Vec<&str> = git_directory_paths
        .iter()
        .filter_map(|git_directory_path| {
            let git_repository_path_option = git_directory_path.strip_suffix("/.git");
            if git_repository_path_option.is_none() {
                log::warn!(
                    path = log::as_serde!(git_directory_path);
                    "Unable to strip '/.git' suffix from path '{}'",
                    git_directory_path
                );
            }
            git_repository_path_option
        })
        .collect();

    write_index_file(git_repository_paths);
}

fn write_index_file(git_repository_paths: Vec<&str>) {
    let home_directory_result = std::env::var("HOME");
    if let Err(error) = home_directory_result {
        log::error!(
            error = log::as_error!(error);
            "Could read HOME environment variable",
        );
        panic!();
    }
    let home_directory = home_directory_result.unwrap();

    let cache_directory_path = std::path::Path::new(&home_directory).join(".cache/centerpiece");
    if let Err(error) = std::fs::create_dir_all(&cache_directory_path) {
        log::error!(
            error = log::as_error!(error);
            "Error while creating cache directory",
        );
        panic!();
    }

    let index_file_path = cache_directory_path.join("git-repositories-index.json");

    let index_file_result = std::fs::File::create(index_file_path);
    if let Err(error) = index_file_result {
        log::error!(
            error = log::as_error!(error);
            "Error while creating index file",
        );
        panic!();
    }
    let index_file = index_file_result.unwrap();

    let mut writer = std::io::BufWriter::new(index_file);
    if let Err(error) = serde_json::to_writer(&mut writer, &git_repository_paths) {
        log::error!(
            error = log::as_error!(error);
            "Error while writing index file",
        );
        panic!();
    }
}
