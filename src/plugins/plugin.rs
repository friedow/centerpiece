use gtk4::gdk::Key;
use gtk4::{Box, Widget};

// pub trait Plugin<T: ObjectExt> {
pub trait Plugin {
    // const NAME: &'static str;

    // fn build_items() -> Vec<T>;
    // fn on_keydown(list_item: &T, key: Key);

    fn get_name(&self) -> String;

    fn build_items(&self) -> Vec<Box>;
    fn on_key_pressed(&self, list_item: &Widget, key: Key);
}
