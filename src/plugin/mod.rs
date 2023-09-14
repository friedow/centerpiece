pub mod clock;

pub enum PluginRequest {
    Search(String),
    Timeout,
}
