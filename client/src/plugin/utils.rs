use fuzzy_matcher::FuzzyMatcher;

pub fn search(entries: Vec<crate::model::Entry>, query: &String) -> Vec<crate::model::Entry> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    if query.is_empty() {
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|entry| entry.title.clone());
        return sorted_entries;
    }

    let mut filtered_entries = entries
        .into_iter()
        .filter_map(|entry| {
            let keywords = format!("{} {}", entry.title, entry.meta);
            let match_result = matcher.fuzzy_indices(&keywords, query);
            if match_result.is_none() {
                return None;
            }
            let (score, _) = match_result.unwrap();
            return Some((score, entry));
        })
        .collect::<Vec<(i64, crate::model::Entry)>>();

    filtered_entries.sort_by(|(a_score, _), (b_score, _)| b_score.cmp(a_score));
    return filtered_entries
        .into_iter()
        .map(|(_, entry)| entry)
        .collect::<Vec<crate::model::Entry>>();
}

pub fn read_index_file<T>(file_name: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let home_directory_result = std::env::var("HOME");
    if let Err(error) = home_directory_result {
        log::error!(
            error = log::as_error!(error);
            "Could not read HOME environment variable",
        );
        std::process::exit(1);
    }
    let home_directory = home_directory_result.unwrap();

    let index_file_path = std::path::Path::new(&home_directory)
        .join(".cache/centerpiece")
        .join(file_name);
    let index_file_result = std::fs::File::open(index_file_path);
    if let Err(error) = index_file_result {
        log::error!(
            error = log::as_error!(error);
            "Error while opening index file",
        );
        std::process::exit(1);
    }
    let index_file = index_file_result.unwrap();

    let reader = std::io::BufReader::new(index_file);
    let git_repository_paths_result: Result<T, _> = serde_json::from_reader(reader);
    if let Err(error) = git_repository_paths_result {
        log::error!(
            error = log::as_error!(error);
            "Error while reading index file",
        );
        std::process::exit(1);
    }
    let git_repository_paths = git_repository_paths_result.unwrap();
    git_repository_paths
}
