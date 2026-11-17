use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct NiriWorkspacesPlugin {
    entries: Vec<crate::model::Entry>,
}

impl NiriWorkspacesPlugin {
    fn entries_from_workspaces(
        mut workspaces: Vec<niri_ipc::Workspace>,
    ) -> Vec<crate::model::Entry> {
        // Same ordering as `niri msg workspaces`: grouped by output, top to bottom.
        workspaces.sort_by(|a, b| a.output.cmp(&b.output).then(a.idx.cmp(&b.idx)));

        let multi_output = workspaces
            .iter()
            .filter_map(|w| w.output.as_deref())
            .collect::<std::collections::HashSet<_>>()
            .len()
            > 1;

        workspaces
            .into_iter()
            .map(|workspace| {
                let name = workspace
                    .name
                    .filter(|n| !n.is_empty())
                    .unwrap_or_else(|| String::from("Unnamed Workspace"));
                let mut title = format!("{} {}", workspace.idx, name);
                if multi_output && let Some(output) = &workspace.output {
                    title.push_str(&format!(" ({output})"));
                }
                crate::model::Entry {
                    id: workspace.id.to_string(),
                    title,
                    action: String::from("focus"),
                    meta: String::from("Niri Workspaces"),
                    command: None,
                }
            })
            .collect()
    }
}

impl Plugin for NiriWorkspacesPlugin {
    fn id() -> &'static str {
        "niri-workspaces"
    }
    fn priority() -> u32 {
        31
    }
    fn title() -> &'static str {
        "󰜩 Workspaces"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        let socket =
            niri_ipc::socket::Socket::connect().context("Failed to connect to niri IPC socket.")?;

        let (reply, _) = socket
            .send(niri_ipc::Request::Workspaces)
            .context("Failed to send Workspaces request to niri IPC.")?;

        let workspaces = match reply {
            Ok(niri_ipc::Response::Workspaces(workspaces)) => workspaces,
            Ok(other) => anyhow::bail!("Unexpected niri IPC response: {:?}", other),
            Err(msg) => anyhow::bail!("Niri IPC error: {}", msg),
        };

        // Keep niri's layout order; `self.sort()` would reorder alphabetically.
        self.set_entries(Self::entries_from_workspaces(workspaces));

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut async_channel::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let workspace_id: u64 = entry
            .id
            .parse()
            .context("Failed to parse workspace id as u64.")?;

        let socket = niri_ipc::socket::Socket::connect()
            .context("Failed to connect to niri IPC socket for focus action.")?;

        let (reply, _) = socket
            .send(niri_ipc::Request::Action(
                niri_ipc::Action::FocusWorkspace {
                    reference: niri_ipc::WorkspaceReferenceArg::Id(workspace_id),
                },
            ))
            .context(format!(
                "Failed to focus workspace while activating entry with id '{}'.",
                entry.id
            ))?;

        if let Err(msg) = reply {
            anyhow::bail!("Niri IPC error focusing workspace: {}", msg);
        }

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
