use crate::plugin::utils::Plugin;

pub struct ClockPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for ClockPlugin {
    fn new() -> Self {
        return Self { entries: vec![] };
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

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let date = chrono::Local::now();
        self.entries = vec![
            crate::model::Entry {
                id: String::from("time-entry"),
                title: date.format("%H:%M:%S").to_string(),
                action: String::from(""),
                meta: String::from("Clock Time"),
                command: None,
            },
            crate::model::Entry {
                id: String::from("date"),
                title: date.format("%A, %_d. %B %Y").to_string(),
                action: String::from(""),
                meta: String::from("Clock Date"),
                command: None,
            },
        ];

        return Ok(());
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }
}
