

use gtk::GtkWindowExt;
use gtk::DialogExt;
use gtk::EntryExt;
use gtk::ToggleButtonExt;
use gtk::WidgetExt;
use gtk::LabelExt;
use gtk::ContainerExt;
use gtk::ButtonExt;
use crate::app::App;


#[derive(Shrinkwrap)]
pub struct ConfigDialog {
    #[shrinkwrap(main_field)]
    dialog: gtk::Dialog,
    location_label: gtk::Label,
    feed_entry: FeedEntry,
    active: gtk::CheckButton,
    preserve_items: gtk::CheckButton

}

impl ConfigDialog {
    pub fn new (parent: &gtk::Window) -> Self {
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

        let location_label = gtk::Label::new(Some("No save location found. Config might not be preserved"));
        location_label.show();

        let feed_entry = FeedEntry::new();
        let active = gtk::CheckButtonBuilder::new()
            .label("Active")
            .name("active")
            .build();
        active.show();
        let preserve_items = gtk::CheckButtonBuilder::new()
            .label("Preserve items")
            .name("preserve-items")
            .build();
        preserve_items.show();

        container.add(&location_label);
        container.add(feed_entry.as_widget());
        container.add(&active);
        container.add(&preserve_items);

        ConfigDialog {
            dialog,
            location_label,
            feed_entry,
            active,
            preserve_items
        }
    }

    pub fn update(&self, app: &App) {
        if let Some(loc) = &app.config.save_location {
            self.location_label.set_label(&loc);
        }
        self.feed_entry.set_text(&app.config.feed);
        self.active.set_active(app.config.active);
    }

    pub fn get_active(&self) -> bool {
        self.active.get_active()
    }

    pub fn get_preserve_items(&self) -> bool {
        self.preserve_items.get_active()
    }

    pub fn get_feed(&self) -> String {
        if let Some(value) = &self.feed_entry.get_text() {
            String::from(value.as_str())
        } else {
            String::new()
        }
    }

    pub fn connect_test_feed<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.feed_entry.connect_test(f);
    }

    pub fn update_test_results(&self, result: Result<(), crate::state::ErrorType>) {
        self.feed_entry.update_test_results(result);
    }

}



#[derive(Shrinkwrap)]
pub struct FeedEntry {
    #[shrinkwrap(main_field)]
    entry: gtk::Entry,
    container: gtk::Box,
    button: gtk::Button,
}

impl FeedEntry {
    pub fn new() -> Self {
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

        FeedEntry {container, entry, button}
    }

    pub fn connect_test<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.button.connect_clicked(move |button| {
            button.set_label("...");
            f(button)
        });
    }

    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn update_test_results(&self, result: Result<(), crate::state::ErrorType>) {
        if let Err(err) = result {
            self.button.set_label("Err");
            self.button.set_tooltip_text(Some(&format!("{}", err)));
        } else {
            self.button.set_label("Ok");
        }
    }
}
