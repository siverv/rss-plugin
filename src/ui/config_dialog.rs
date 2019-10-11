

use gtk::GtkWindowExt;
use gtk::DialogExt;
use gtk::WidgetExt;
use gtk::LabelExt;
use gtk::ContainerExt;


pub fn open_config_dialog(parent: &gtk::Window) -> (gtk::Dialog, gtk::Box) {
    let dialog = gtk::Dialog::new_with_buttons(
        Some("Rss Plugin Configuration"),
        Some(parent),
        gtk::DialogFlags::DESTROY_WITH_PARENT,
        &[
            ("gtk-close", gtk::ResponseType::Close),
            ("gtk-save", gtk::ResponseType::Accept)
        ]
    );
    dialog.set_position(gtk::WindowPosition::Center);
    dialog.set_icon_name(Some("xfce4-settings"));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
    dialog.get_content_area().add(&container);
    container.show();

    dialog.connect_response(|dialog, response| {
        match response {
            gtk::ResponseType::Accept => {},
            _ => {dialog.destroy();}
        }
    });
    
    (dialog, container)
}


pub fn new_feed_entry() -> (gtk::Box, gtk::Entry, gtk::Button) {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    container.show();

    let label = gtk::LabelBuilder::new()
        .label("Feed URI:")
        .build();
    label.show();
    container.add(&label);

    let entry = gtk::EntryBuilder::new()
        .placeholder_text("https://...")
        .build();
    entry.show();
    container.add(&entry);
 
    let button = gtk::ButtonBuilder::new()
        .label("Test")
        .build();
    button.show();
    container.add(&button);

    (container, entry, button)
}


pub fn new_header(save_location: &Option<String>) -> gtk::Label {

    let label = gtk::Label::new(Some("No save location found. Config might not be preserved"));
    if let Some(loc) = save_location {
        label.set_label(&loc);
    }
    label.show();

    label
}


pub fn new_active_field() -> gtk::CheckButton {
    let active = gtk::CheckButtonBuilder::new()
        .label("Active")
        .name("active")
        .build();
    active.show();
    active
}
