#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)]
enum Section {
    #[serde(rename_all = "PascalCase")]
    Profile {
        name: String,
        is_relative: String,
        path: String,
        default: Option<String>,
    },
    #[serde(rename_all = "PascalCase")]
    General {
        start_with_last_profile: String,
        version: Option<String>,
    },

    #[serde(rename_all = "PascalCase")]
    Install { default: String, locked: String },
}

pub fn profile_path() -> anyhow::Result<String> {
    let home_directory = std::env::var("HOME")?;

    let profiles_file_path = format!("{home_directory}/.mozilla/firefox/profiles.ini");
    let profiles_file = std::fs::File::open(profiles_file_path)?;
    let profiles_file_contents: std::collections::HashMap<String, Section> =
        serde_ini::from_read(profiles_file)?;

    let mut default_profile = profiles_file_contents
        .values()
        .find(|section| match section {
            Section::Profile { default, .. } => {
                default.clone().unwrap_or(String::from("")) == String::from("1")
            }
            _ => false,
        });

    if default_profile.is_none() {
        default_profile = profiles_file_contents
            .values()
            .find(|section| match section {
                Section::Profile { .. } => true,
                _ => false,
            });
    }

    if default_profile.is_none() {
        return Err(anyhow::anyhow!("Could not find a firefox profile."));
    }

    match default_profile.unwrap() {
        Section::Profile {
            is_relative, path, ..
        } => {
            if is_relative.eq(&String::from("1")) {
                Ok(format!("{home_directory}/.mozilla/firefox/{path}"))
            } else {
                Ok(path.clone())
            }
        }
        _ => {
            unreachable!("A non-profile section should be parsed as a profile.");
        }
    }
}
