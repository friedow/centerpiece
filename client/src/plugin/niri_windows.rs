use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct NiriWindowsPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for NiriWindowsPlugin {
    fn id() -> &'static str {
        "niri-windows"
    }
    fn priority() -> u32 {
        30
    }
    fn title() -> &'static str {
        "󰖯 Windows"
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
            .send(niri_ipc::Request::Windows)
            .context("Failed to send Windows request to niri IPC.")?;

        let windows = match reply {
            Ok(niri_ipc::Response::Windows(windows)) => windows,
            Ok(other) => anyhow::bail!("Unexpected niri IPC response: {:?}", other),
            Err(msg) => anyhow::bail!("Niri IPC error: {}", msg),
        };

        let entries: Vec<crate::model::Entry> = windows
            .into_iter()
            .map(|window| {
                let title = window
                    .title
                    .filter(|t| !t.is_empty())
                    .or(window.app_id.clone())
                    .unwrap_or_else(|| String::from("-- window name missing --"));
                crate::model::Entry {
                    id: window.id.to_string(),
                    title,
                    action: String::from("focus"),
                    meta: String::from("Niri Windows"),
                    command: None,
                }
            })
            .collect();

        self.set_entries(entries);
        self.sort();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut async_channel::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let window_id: u64 = entry
            .id
            .parse()
            .context("Failed to parse window id as u64.")?;

        let socket = niri_ipc::socket::Socket::connect()
            .context("Failed to connect to niri IPC socket for focus action.")?;

        let (reply, _) = socket
            .send(niri_ipc::Request::Action(niri_ipc::Action::FocusWindow {
                id: window_id,
            }))
            .context(format!(
                "Failed to focus window while activating entry with id '{}'.",
                entry.id
            ))?;

        if let Err(msg) = reply {
            anyhow::bail!("Niri IPC error focusing window: {}", msg);
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
