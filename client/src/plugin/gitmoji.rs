use crate::plugin::utils::Plugin;
use anyhow::Context;

struct Gitmoji {
    emoji: String,
    description: String,
}
fn gitmojis() -> Vec<Gitmoji> {
    vec![
        Gitmoji {
            emoji: String::from("🎨"),
            description: String::from("🎨 Improve structure / format of the code."),
        },
        Gitmoji {
            emoji: String::from("⚡️"),
            description: String::from("⚡️ Improve performance."),
        },
        Gitmoji {
            emoji: String::from("🔥"),
            description: String::from("🔥 Remove code or files."),
        },
        Gitmoji {
            emoji: String::from("🐛"),
            description: String::from("🐛 Fix a bug."),
        },
        Gitmoji {
            emoji: String::from("🚑️"),
            description: String::from("🚑️ Critical hotfix."),
        },
        Gitmoji {
            emoji: String::from("✨"),
            description: String::from("✨ Introduce new features."),
        },
        Gitmoji {
            emoji: String::from("📝"),
            description: String::from("📝 Add or update documentation."),
        },
        Gitmoji {
            emoji: String::from("🚀"),
            description: String::from("🚀 Deploy stuff."),
        },
        Gitmoji {
            emoji: String::from("💄"),
            description: String::from("💄 Add or update the UI and style files."),
        },
        Gitmoji {
            emoji: String::from("🎉"),
            description: String::from("🎉 Begin a project."),
        },
        Gitmoji {
            emoji: String::from("✅"),
            description: String::from("✅ Add, update, or pass tests."),
        },
        Gitmoji {
            emoji: String::from("🔒️"),
            description: String::from("🔒️ Fix security or privacy issues."),
        },
        Gitmoji {
            emoji: String::from("🔐"),
            description: String::from("🔐 Add or update secrets."),
        },
        Gitmoji {
            emoji: String::from("🔖"),
            description: String::from("🔖 Release / Version tags."),
        },
        Gitmoji {
            emoji: String::from("🚨"),
            description: String::from("🚨 Fix compiler / linter warnings."),
        },
        Gitmoji {
            emoji: String::from("🚧"),
            description: String::from("🚧 Work in progress."),
        },
        Gitmoji {
            emoji: String::from("💚"),
            description: String::from("💚 Fix CI Build."),
        },
        Gitmoji {
            emoji: String::from("⬇️"),
            description: String::from("⬇️ Downgrade dependencies."),
        },
        Gitmoji {
            emoji: String::from("⬆️"),
            description: String::from("⬆️ Upgrade dependencies."),
        },
        Gitmoji {
            emoji: String::from("📌"),
            description: String::from("📌 Pin dependencies to specific versions."),
        },
        Gitmoji {
            emoji: String::from("👷"),
            description: String::from("👷 Add or update CI build system."),
        },
        Gitmoji {
            emoji: String::from("📈"),
            description: String::from("📈 Add or update analytics or track code."),
        },
        Gitmoji {
            emoji: String::from("♻️"),
            description: String::from("♻️ Refactor code."),
        },
        Gitmoji {
            emoji: String::from("➕"),
            description: String::from("➕ Add a dependency."),
        },
        Gitmoji {
            emoji: String::from("➖"),
            description: String::from("➖ Remove a dependency."),
        },
        Gitmoji {
            emoji: String::from("🔧"),
            description: String::from("🔧 Add or update configuration files."),
        },
        Gitmoji {
            emoji: String::from("🔨"),
            description: String::from("🔨 Add or update development scripts."),
        },
        Gitmoji {
            emoji: String::from("🌐"),
            description: String::from("🌐 Internationalization and localization."),
        },
        Gitmoji {
            emoji: String::from("✏️"),
            description: String::from("✏️ Fix typos."),
        },
        Gitmoji {
            emoji: String::from("💩"),
            description: String::from("💩 Write bad code that needs to be improved."),
        },
        Gitmoji {
            emoji: String::from("⏪️"),
            description: String::from("⏪️ Revert changes."),
        },
        Gitmoji {
            emoji: String::from("🔀"),
            description: String::from("🔀 Merge branches."),
        },
        Gitmoji {
            emoji: String::from("📦️"),
            description: String::from("📦️ Add or update compiled files or packages."),
        },
        Gitmoji {
            emoji: String::from("👽️"),
            description: String::from("👽️ Update code due to external API changes."),
        },
        Gitmoji {
            emoji: String::from("🚚"),
            description: String::from("🚚 Move or rename resources (e.g.): files, paths, routes)."),
        },
        Gitmoji {
            emoji: String::from("📄"),
            description: String::from("📄 Add or update license."),
        },
        Gitmoji {
            emoji: String::from("💥"),
            description: String::from("💥 Introduce breaking changes."),
        },
        Gitmoji {
            emoji: String::from("🍱"),
            description: String::from("🍱 Add or update assets."),
        },
        Gitmoji {
            emoji: String::from("♿️"),
            description: String::from("♿️ Improve accessibility."),
        },
        Gitmoji {
            emoji: String::from("💡"),
            description: String::from("💡 Add or update comments in source code."),
        },
        Gitmoji {
            emoji: String::from("🍻"),
            description: String::from("🍻 Write code drunkenly."),
        },
        Gitmoji {
            emoji: String::from("💬"),
            description: String::from("💬 Add or update text and literals."),
        },
        Gitmoji {
            emoji: String::from("🗃️"),
            description: String::from("🗃️ Perform database related changes."),
        },
        Gitmoji {
            emoji: String::from("🔊"),
            description: String::from("🔊 Add or update logs."),
        },
        Gitmoji {
            emoji: String::from("🔇"),
            description: String::from("🔇 Remove logs."),
        },
        Gitmoji {
            emoji: String::from("👥"),
            description: String::from("👥 Add or update contributor(s)."),
        },
        Gitmoji {
            emoji: String::from("🚸"),
            description: String::from("🚸 Improve user experience / usability."),
        },
        Gitmoji {
            emoji: String::from("🏗️"),
            description: String::from("🏗️ Make architectural changes."),
        },
        Gitmoji {
            emoji: String::from("📱"),
            description: String::from("📱 Work on responsive design."),
        },
        Gitmoji {
            emoji: String::from("🤡"),
            description: String::from("🤡 Mock things."),
        },
        Gitmoji {
            emoji: String::from("🥚"),
            description: String::from("🥚 Add or update an easter egg."),
        },
        Gitmoji {
            emoji: String::from("🙈"),
            description: String::from("🙈 Add or update a .gitignore file."),
        },
        Gitmoji {
            emoji: String::from("📸"),
            description: String::from("📸 Add or update snapshots."),
        },
        Gitmoji {
            emoji: String::from("⚗️"),
            description: String::from("⚗️ Perform experiments."),
        },
        Gitmoji {
            emoji: String::from("🔍️"),
            description: String::from("🔍️ Improve SEO."),
        },
        Gitmoji {
            emoji: String::from("🏷️"),
            description: String::from("🏷️ Add or update types."),
        },
        Gitmoji {
            emoji: String::from("🌱"),
            description: String::from("🌱 Add or update seed files."),
        },
        Gitmoji {
            emoji: String::from("🚩"),
            description: String::from("🚩 Add, update, or remove feature flags."),
        },
        Gitmoji {
            emoji: String::from("🥅"),
            description: String::from("🥅 Catch errors."),
        },
        Gitmoji {
            emoji: String::from("💫"),
            description: String::from("💫 Add or update animations and transitions."),
        },
        Gitmoji {
            emoji: String::from("🗑️"),
            description: String::from("🗑️ Deprecate code that needs to be cleaned up."),
        },
        Gitmoji {
            emoji: String::from("🛂"),
            description: String::from(
                "🛂 Work on code related to authorization, roles and permissions.",
            ),
        },
        Gitmoji {
            emoji: String::from("🩹"),
            description: String::from("🩹 Simple fix for a non-critical issue."),
        },
        Gitmoji {
            emoji: String::from("🧐"),
            description: String::from("🧐 Data exploration/inspection."),
        },
        Gitmoji {
            emoji: String::from("⚰️"),
            description: String::from("⚰️ Remove dead code."),
        },
        Gitmoji {
            emoji: String::from("🧪"),
            description: String::from("🧪 Add a failing test."),
        },
        Gitmoji {
            emoji: String::from("👔"),
            description: String::from("👔 Add or update business logic."),
        },
        Gitmoji {
            emoji: String::from("🩺"),
            description: String::from("🩺 Add or update healthcheck."),
        },
        Gitmoji {
            emoji: String::from("🧱"),
            description: String::from("🧱 Infrastructure related changes."),
        },
        Gitmoji {
            emoji: String::from("🧑‍💻"),
            description: String::from("🧑‍💻 Improve developer experience."),
        },
        Gitmoji {
            emoji: String::from("💸"),
            description: String::from("💸 Add sponsorships or money related infrastructure."),
        },
        Gitmoji {
            emoji: String::from("🧵"),
            description: String::from(
                "🧵 Add or update code related to multithreading or concurrency.",
            ),
        },
        Gitmoji {
            emoji: String::from("🦺"),
            description: String::from("🦺 Add or update code related to validation."),
        },
    ]
}

pub struct GitmojiPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for GitmojiPlugin {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn id() -> &'static str {
        "gitmoji"
    }

    fn priority() -> u32 {
        13
    }

    fn title() -> &'static str {
        "󰞅 Gitmoji"
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let entries = gitmojis()
            .iter()
            .map(|gitmoji| crate::model::Entry {
                id: gitmoji.emoji.clone(),
                title: gitmoji.description.clone(),
                action: String::from("copy"),
                meta: String::from("Gitmoji"),
                command: None,
            })
            .collect();

        self.set_entries(entries);
        Ok(())
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut async_channel::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("wl-copy")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to copy gitmoji while activating entry with id '{}'.",
                entry.id
            ))?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
