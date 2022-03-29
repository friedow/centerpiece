use battery::units::ratio::percent;
use glib::ObjectExt;
use gtk4::gdk::Key;
use gtk4::{Box, Widget};
use std::error::Error;
use subprocess::Exec;
use walkdir::{DirEntry, WalkDir};

use crate::components::list_item;
use crate::plugins::plugin::Plugin;

pub struct SystemMonitorPlugin();

impl Plugin for SystemMonitorPlugin {
    fn get_name(&self) -> String {
        return String::from("System Monitor");
    }

    fn build_items(&self) -> Vec<Box> {
        let battery_charge = get_battery_charge_level().unwrap();
        let home = std::env::var("HOME").unwrap();
        let git_directories = WalkDir::new(home)
            .into_iter()
            .filter_entry(is_dir)
            .filter_map(|e| e.ok())
            .filter(is_git_repository);

        return git_directories.map(to_list_item).collect();
    }

    fn on_key_pressed(&self, list_item: &Widget, key: Key) {
        if key.name().unwrap() == "Return" {
            unsafe {
                let path = list_item.data::<String>("path").unwrap().as_ref();
                let _result = Exec::cmd("code").arg(path).join();
            }
        }
    }
}

fn get_battery_charge_level() -> Result<f32, battery::Error> {
    let manager = battery::Manager::new()?;
    let batteryOption = manager.batteries()?.next();

    if batteryOption.is_none() {
        return Err(battery::Error::from(Error::from("No battery found.")));
    }

    let percentage = battery.state_of_charge().get::<percent>();
    println!("Charge Level: {:?}%", percentage);
    return Ok(percentage);
    // return Result::from(percentage);

    // for (idx, maybe_battery) in manager.batteries()?.enumerate() {
    //     let battery = maybe_battery?;

    //     let percentage = battery.state_of_charge().get::<percent>();
    //     println!("Battery #{}:", idx);
    //     println!("B: {:?}", battery);
    //     println!("Charge Level: {:?}%", percentage);
    //     println!("Vendor: {:?}", battery.vendor());
    //     println!("Model: {:?}", battery.model());
    //     println!("State: {:?}", battery.state());
    //     println!("Time to full charge: {:?}", battery.time_to_full());
    //     println!("");
    // }
}

fn is_dir(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
    return is_dir && (!is_hidden || is_git_repository(entry));
}

fn is_git_repository(entry: &DirEntry) -> bool {
    return entry.file_name().to_str().unwrap().eq(".git");
}

fn to_list_item(entry: DirEntry) -> Box {
    let path = entry.path().display().to_string().replace("/.git", "");

    let home = std::env::var("HOME").unwrap();
    let title = path.replace(&home, "~");

    let action_text = String::from("open");

    let list_item = list_item::build(title, action_text);
    unsafe { list_item.set_data("path", path) }
    return list_item;
}
