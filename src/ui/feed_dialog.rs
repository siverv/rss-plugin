

use gtk::ContainerExt;
use gtk::DialogExt;
use gtk::WidgetExt;
use gtk::ButtonExt;
use gdk;

pub fn open_feed_dialog(parent: &gtk::Window) -> (gtk::Dialog, gtk::Box) {
    let dialog = gtk::DialogBuilder::new()
        .type_(gtk::WindowType::Toplevel)
        .title("RSS Feed")
        .decorated(false)
        .resizable(false)
        .transient_for(parent)
        .destroy_with_parent(true)
        .skip_pager_hint(true)
        .skip_taskbar_hint(true)
        .events(gdk::EventMask::FOCUS_CHANGE_MASK)
        .window_position(gtk::WindowPosition::Mouse)
        .width_request(200)
        .height_request(400)
        .build();

    dialog.connect_focus_out_event(|dialog, _| {
        dialog.destroy();
        gtk::Inhibit(true)
    });
    dialog.show();
    dialog.grab_focus();

    let (scrolling_area, container) = new_feed_list();
    dialog.get_content_area().add(&scrolling_area);

    (dialog, container)
}



pub fn new_feed_item(item: &rss::Item) -> gtk::Box {
    let item_widget = gtk::BoxBuilder::new()
        .hexpand(true)
        .vexpand(true)
        .build();
    if let Some(link) = item.link() {
        let title = gtk::LinkButton::new(link);
        title.set_label(item.title().unwrap_or("N/A"));
        title.show();
        item_widget.add(&title);
    } else {
        let title = gtk::Label::new(item.title());
        title.show();
        item_widget.add(&title);
    }
    item_widget.show();
    item_widget
}

pub fn new_feed_list() -> (gtk::ScrolledWindow, gtk::Box) {
    let scrolling_area = gtk::ScrolledWindowBuilder::new()
        .vexpand(true)
        .hexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();
    scrolling_area.show();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    scrolling_area.add(&container);
    container.show();
    
    (scrolling_area, container)
}
