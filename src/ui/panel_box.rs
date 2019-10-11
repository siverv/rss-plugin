

use gtk::ContainerExt;
use gtk::WidgetExt;


pub fn new_panel_box() -> (gtk::EventBox, gtk::Box, gtk::Label) {
    let event_box = gtk::EventBox::new();
    event_box.show();

    let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    container.show();
    event_box.add(&container);

    let label = gtk::Label::new(Some(""));
    //label.show();
    container.add(&label);

    (event_box, container, label)

}
