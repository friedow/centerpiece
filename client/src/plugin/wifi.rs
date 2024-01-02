use std::println;

use anyhow::Context;
use dbus::blocking::Connection;
use networkmanager::devices::{Device, Wireless};
use networkmanager::NetworkManager;

use crate::plugin::utils::Plugin;

pub struct WifiPlugin {
    entries: Vec<crate::model::Entry>,
}

impl WifiPlugin {
    fn get_access_point_entries(&self) -> Result<Vec<crate::model::Entry>, networkmanager::Error> {
        let dbus_connection = Connection::new_system()?;
        let nm = NetworkManager::new(&dbus_connection);
        let device = nm.get_device_by_ip_iface("wlo1")?;

        match device {
            Device::WiFi(wifi_device) => {
                wifi_device.request_scan(std::collections::HashMap::new())?;
                let mut access_points = wifi_device.get_access_points()?;
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

                        println!("{}", access_point.wpa_flags().ok()?);
                        return Some(crate::model::Entry {
                            id: ssid.clone(),
                            title: format!("{} {}", strength_icon, ssid),
                            action: String::from("connect"),
                            meta: String::from("wifi wlan wireless lan"),
                            command: None,
                        });
                    })
                    .collect();
                return Ok(wifi_network_entries);
            }
            _ => {}
        }

        return Err(networkmanager::Error::UnsupportedDevice);
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

        let access_point_entries_result = self.get_access_point_entries();
        if let Err(error) = access_point_entries_result {
            println!("{:?}", error);
            return Err(anyhow::anyhow!("Failed to get access points."));
        }

        self.entries = access_point_entries_result.unwrap();

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
