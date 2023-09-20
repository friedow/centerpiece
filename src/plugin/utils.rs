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

    filtered_entries.sort_by_key(|(score, _)| score.clone());
    filtered_entries.reverse();
    return filtered_entries
        .into_iter()
        .map(|(_, entry)| entry)
        .collect::<Vec<crate::model::Entry>>();
}
