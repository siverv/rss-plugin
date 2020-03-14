

use gtk::ContainerExt;
use gtk::DialogExt;
use gtk::WidgetExt;
use gtk::ButtonExt;
use gtk::LinkButtonExt;
use gtk::LabelExt;
use gdk;



#[derive(Shrinkwrap)]
pub struct FeedDialog {
    #[shrinkwrap(main_field)]
    dialog: gtk::Dialog,
    feed_list: FeedList
}

impl FeedDialog {
    pub fn new(parent: &gtk::Window) -> Self {
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

        let feed_list = FeedList::new();
        dialog.get_content_area().add(feed_list.as_widget());
        feed_list.show();

        FeedDialog {dialog, feed_list}
    }

    pub fn set_items(&mut self, items: &Vec<rss::Item>) {
        self.feed_list.update_with_items(items);
        self.feed_list.show();
    }
}



#[derive(Shrinkwrap)]
pub struct FeedItem {
    #[shrinkwrap(main_field)]
    container: gtk::Box,
    title_link: Option<gtk::LinkButton>,
    title_label: Option<gtk::Label>,
    item: Option<rss::Item>
}

impl FeedItem {
    pub fn new() -> Self {
        let container = gtk::BoxBuilder::new()
            .hexpand(true)
            .vexpand(true)
            .build();
        container.show();
        FeedItem {
            container,
            title_link: None,
            title_label: None,
            item: None
        }
    }
    
    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn set_item(&mut self, item: &rss::Item) {
        if let Some(link) = item.link() {
            if let Some(widget) = &self.title_label {
                widget.hide();
            }
            if self.title_link.is_none() {
                let widget = gtk::LinkButton::new(link);
                self.container.add(&widget);
                self.title_link = Some(widget);
            }
            if let Some(widget) = &self.title_link {
                widget.set_uri(link);
                widget.set_label(item.title().unwrap_or("N/A"));
                widget.show();
            }
        } else {
            if let Some(widget) = &self.title_link {
                widget.hide();
            }
            if self.title_label.is_none() {
                let widget = gtk::Label::new(item.title());
                self.container.add(&widget);
                self.title_label = Some(widget);
            }
            if let Some(widget) = &self.title_label {
                widget.set_label(item.title().unwrap_or("N/A"));
                widget.show();
            }
        }
        self.item = Some(item.clone());
    }
}



#[derive(Shrinkwrap)]
pub struct FeedList {
    #[shrinkwrap(main_field)]
    scrolling_area: gtk::ScrolledWindow,
    container: gtk::Box,
    feed_items: Vec<FeedItem>
}

impl FeedList {
    pub fn new() -> Self {
        let scrolling_area = gtk::ScrolledWindowBuilder::new()
            .vexpand(true)
            .hexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        scrolling_area.show();
    
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
        scrolling_area.add(&container);
        container.show();
        FeedList {
            container,
            scrolling_area,
            feed_items: Vec::default()
        }
    }

    pub fn as_widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolling_area
    }

    pub fn update_with_items(&mut self, items: &Vec<rss::Item>) {
        let items_len = items.len();
        let feed_items_len = self.feed_items.len();
        if items_len < feed_items_len {
            for _ in (feed_items_len-1)..items_len {
                self.feed_items.pop().map(|w| w.destroy());
            }
        } else if items_len > feed_items_len {
            for _ in feed_items_len..items_len {
                let feed_item = FeedItem::new();
                self.container.add(feed_item.as_widget());
                self.feed_items.push(feed_item);
            }
        }
        items.iter().enumerate().for_each(|(i, item)| {
            self.feed_items.get_mut(i).map(|w| {
                w.set_item(item);
                w.show();
            });
        });
    }
}
