use crate::plugin::utils::Plugin;

pub struct ClockPlugin {}

impl Plugin for ClockPlugin {
    fn new() -> Self {
        return Self {};
    }

    fn id() -> &'static str {
        return "clock";
    }

    fn priority() -> u32 {
        return 10;
    }

    fn title() -> &'static str {
        return "󰅐 Clock";
    }

    fn update_timeout() -> Option<std::time::Duration> {
        return Some(std::time::Duration::from_secs(1));
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        let date = chrono::Local::now();
        return vec![
            crate::model::Entry {
                id: String::from("time-entry"),
                title: date.format("%H:%M:%S").to_string(),
                action: String::from(""),
                meta: String::from("Clock Time"),
            },
            crate::model::Entry {
                id: String::from("date"),
                title: date.format("%A, %_d. %B %Y").to_string(),
                action: String::from(""),
                meta: String::from("Clock Date"),
            },
        ];
    }
}
