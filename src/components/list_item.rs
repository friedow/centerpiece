use gtk4::{prelude::*, Box, CenterBox, Label, Orientation};

pub fn build(title: String, action_text: String) -> Box {
    let title_label = Label::new(Some(&title));
    let action_label = Label::new(Some(&action_text));

    let hbox = CenterBox::new();
    hbox.set_start_widget(Some(&title_label));
    hbox.set_end_widget(Some(&action_label));
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_end(8);
    hbox.set_margin_start(8);

    let wrapper = Box::new(Orientation::Vertical, 0);
    wrapper.append(&hbox);
    return wrapper;
}
