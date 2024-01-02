use std::println;

use anyhow::Context;
use dbus::blocking::Connection;
use networkmanager::devices::{Any, Device, WiFiDevice, Wired, Wireless};
use networkmanager::{Error, NetworkManager};

use crate::plugin::utils::Plugin;

pub struct WifiPlugin {
    entries: Vec<crate::model::Entry>,
}

impl WifiPlugin {
    fn get_access_point_entries(&self) -> Result<Vec<crate::model::Entry>, networkmanager::Error> {
        let dbus_connection = Connection::new_system()?;
        println!("---- dbus constructed");
        let nm = NetworkManager::new(&dbus_connection);
        println!("---- nm constructed");
        let device = nm.get_device_by_ip_iface("wlo1")?;

        match device {
            Device::WiFi(wifi_device) => {
                println!("---- wwifi device found");
                wifi_device.request_scan(std::collections::HashMap::new())?;
                return Ok(wifi_device
                    .get_access_points()?
                    .into_iter()
                    .filter_map(|access_point| {
                        let ssid = access_point.ssid().ok()?;
                        return Some(crate::model::Entry {
                            id: ssid.clone(),
                            title: ssid,
                            action: String::from("connect"),
                            meta: String::from("Wifi WLAN Wireless Lan"),
                            command: None,
                        });
                    })
                    .collect());
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

        println!("TEST===================================================================================");
        let access_point_entries_result = self.get_access_point_entries();
        if let Err(error) = access_point_entries_result {
            println!("{:?}", error);
            return Err(anyhow::anyhow!("Toast"));
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
