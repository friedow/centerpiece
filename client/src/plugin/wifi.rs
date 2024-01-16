use anyhow::{anyhow, Context, Result};
use dbus::blocking::Connection;
use networkmanager::devices::{Device, Wireless};
use networkmanager::NetworkManager;
use std::matches;

use crate::plugin::utils::Plugin;

pub struct WifiPlugin {
    entries: Vec<crate::model::Entry>,
}

impl WifiPlugin {
    fn get_access_point_entries(&self) -> Result<Vec<crate::model::Entry>> {
        // get wifi device
        let dbus_connection = Connection::new_system()?;
        let nm = NetworkManager::new(&dbus_connection);
        let devices = nm
            .get_devices()
            .map_err(|_| anyhow!("Unable to get network devices."))?;

        let first_wifi_device = devices
            .into_iter()
            .find(|device| matches!(device, Device::WiFi(_)))
            .ok_or(anyhow!("Unable to find a wifi network device."))?;

        let wifi_device = match first_wifi_device {
            Device::WiFi(wifi_device) => wifi_device,
            _ => unreachable!("The found wifi network device is no wifi network device."),
        };

        // get access points
        wifi_device
            .request_scan(std::collections::HashMap::new())
            .map_err(|_| anyhow!("Failed to request scan for wifi access points."))?;
        let mut access_points = wifi_device
            .get_access_points()
            .map_err(|_| anyhow!("Failed to get access points from wifi device."))?;

        let active_access_point_ssid = match wifi_device.active_access_point() {
            Ok(access_point) => access_point.ssid().unwrap_or(String::new()),
            Err(_) => String::new(),
        };

        // dedup access points by name and sort by signal strengh
        access_points.sort_by_key(|access_point| access_point.strength().ok().unwrap());
        access_points.reverse();
        access_points.sort_by_key(|access_point| access_point.ssid().ok().unwrap());
        access_points.dedup_by_key(|access_point| access_point.ssid().ok().unwrap());
        access_points.sort_by_key(|access_point| access_point.strength().ok().unwrap());
        access_points.reverse();

        let wifi_network_entries: Vec<crate::model::Entry> = access_points
            .into_iter()
            .filter_map(|access_point| {
                let ssid = access_point.ssid().ok()?;
                let strength = access_point.strength().ok()?;

                let strength_icon = match access_point.rsn_flags().ok()? {
                    0 => match strength {
                        0..=20 => "󰤯",
                        21..=40 => "󰤟",
                        41..=60 => "󰤢",
                        61..=80 => "󰤥",
                        81..=100 => "󰤨",
                        _ => "󰤫",
                    },
                    _ => match strength {
                        0..=20 => "󰤬",
                        21..=40 => "󰤡",
                        41..=60 => "󰤤",
                        61..=80 => "󰤧",
                        81..=100 => "󰤪",
                        _ => "󰤫",
                    },
                };

                let connected_icon = match active_access_point_ssid == ssid {
                    true => String::from(" 󰄬"),
                    false => String::new(),
                };

                return Some(crate::model::Entry {
                    id: ssid.clone(),
                    title: format!("{}{} {}", strength_icon, connected_icon, ssid.clone()),
                    action: String::from("connect"),
                    meta: String::from("wifi wlan wireless lan"),
                    command: Some(vec![
                        String::from("nmcli"),
                        String::from("device"),
                        String::from("wifi"),
                        String::from("connect"),
                        ssid,
                    ]),
                });
            })
            .collect();
        return Ok(wifi_network_entries);
    }
}

impl Plugin for WifiPlugin {
    fn new() -> Self {
        return Self { entries: vec![] };
    }

    fn id() -> &'static str {
        return "wifi";
    }

    fn priority() -> u32 {
        return 18;
    }

    fn title() -> &'static str {
        return "󰖩 Wifi";
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        self.entries = self.get_access_point_entries()?;
        return Ok(());
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        let command = entry.command.context(format!(
            "Failed to unpack command while activating entry with id '{}'.",
            entry.id
        ))?;
        std::process::Command::new(&command[0])
            .args(&command[1..])
            .spawn()?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        return Ok(());
    }
}
