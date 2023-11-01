use crate::plugin::utils::Plugin;
use anyhow::Context;
use std::ops::Rem;
use sysinfo::SystemExt;

pub struct BatteryPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for BatteryPlugin {
    fn id() -> &'static str {
        return "battery";
    }

    fn priority() -> u32 {
        return 14;
    }

    fn title() -> &'static str {
        return "󰁼 Battery";
    }

    fn update_timeout() -> Option<std::time::Duration> {
        return Some(std::time::Duration::from_secs(2));
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let batteries = battery::Manager::new()
            .context("Failed to create battery manager.")?
            .batteries()
            .context("Failed to list batteries using the battery manager.")?;

        for battery_result in batteries {
            let battery =
                battery_result.context("Failed to get battery using the batteries iterator.")?;

            let state_of_charge = battery.state_of_charge() * 100.0;

            let time_to_full_remaining = match battery.time_to_full() {
                Some(time_to_full) => to_display(time_to_full),
                None => String::new(),
            };

            let time_to_empty_remaining = match battery.time_to_empty() {
                Some(time_to_empty) => to_display(time_to_empty),
                None => String::new(),
            };

            let title = format!(
                "{state_of_charge:.0?}% – {state}{time_to_full_remaining}{time_to_empty_remaining}",
                state = battery.state(),
            );

            self.entries.push(crate::model::Entry {
                id: String::from("battery"),
                title,
                action: String::from(""),
                meta: String::from("Resource Monitor Battery"),
            });
        }

        return Ok(());
    }

    fn new() -> Self {
        return Self { entries: vec![] };
    }
}

fn to_display(time_to_empty: battery::units::Time) -> String {
    let mut formatted_time_remaining = String::from(":");
    let hours = (time_to_empty.value / 60.0 / 60.0).round();
    if hours > 0.0 {
        formatted_time_remaining.push_str(format!(" {hours:.0}h").as_str())
    }
    let minutes = (time_to_empty.value / 60.0).rem(60.0).round();
    if minutes > 0.0 {
        formatted_time_remaining.push_str(format!(" {minutes:.0}m").as_str())
    }
    formatted_time_remaining.push_str(" remaining");
    formatted_time_remaining
}
