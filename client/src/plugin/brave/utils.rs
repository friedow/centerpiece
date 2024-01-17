use anyhow::Context;

#[derive(serde::Deserialize)]
struct BookmarksFile {
    roots: BookmarksRoot,
}

#[derive(serde::Deserialize)]
struct BookmarksRoot {
    bookmark_bar: Bookmark,
    other: Bookmark,
    synced: Bookmark,
}
impl From<BookmarksRoot> for Bookmark {
    fn from(val: BookmarksRoot) -> Self {
        Bookmark::Folder(FolderBookark {
            name: String::from("roots"),
            children: vec![val.bookmark_bar, val.other, val.synced],
        })
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Bookmark {
    Folder(FolderBookark),
    Url(UrlBookmark),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FolderBookark {
    name: String,
    children: Vec<Bookmark>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UrlBookmark {
    name: String,
    url: String,
}

impl From<&UrlBookmark> for crate::model::Entry {
    fn from(val: &UrlBookmark) -> Self {
        crate::model::Entry {
            id: val.url.clone(),
            title: val.name.clone(),
            action: String::from("open"),
            meta: String::from("Bookmarks"),
            command: None,
        }
    }
}

impl Bookmark {
    pub fn find_bookmarks_folder_recursive(&self, folder_name: &String) -> Option<&Bookmark> {
        match self {
            Bookmark::Folder(folder) => {
                if &folder.name == folder_name {
                    return Some(self);
                }

                for child in folder.children.iter() {
                    let find_bookmarks_option = child.find_bookmarks_folder_recursive(folder_name);
                    if find_bookmarks_option.is_some() {
                        return find_bookmarks_option;
                    }
                }
                None
            }

            Bookmark::Url(_bookmark) => None,
        }
    }

    pub fn get_bookmarks_recursive(&self, exclude_folders: &Vec<String>) -> Vec<&UrlBookmark> {
        match self {
            Bookmark::Folder(folder) => {
                if exclude_folders.contains(&folder.name) {
                    return vec![];
                }
                return folder
                    .children
                    .iter()
                    .flat_map(|b| b.get_bookmarks_recursive(exclude_folders))
                    .collect();
            }

            Bookmark::Url(url_bookmark) => vec![url_bookmark],
        }
    }
}

pub fn read_bookmarks_file() -> anyhow::Result<Bookmark> {
    let home_directory =
        std::env::var("HOME").context("Could not read HOME environment variable.")?;

    let index_file_path = std::path::Path::new(&home_directory)
        .join(".config/BraveSoftware/Brave-Browser/Default/Bookmarks");

    let bookmarks_file = std::fs::File::open(index_file_path)
        .context("Error while opening brave bookmarks file.")?;
    let reader = std::io::BufReader::new(bookmarks_file);
    let bookmarks_file_content: BookmarksFile =
        serde_json::from_reader(reader).context("Error while reading brave bookmarks file.")?;

    Ok(bookmarks_file_content.roots.into())
}
