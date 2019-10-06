
use gtk::prelude::*;
use gtk::OrientableExt;
use gtk::DialogExt;
use crate::xfce::plugin::*;
use crate::state::{State, ErrorType as StateError};

use std::sync::Arc;
use std::cell::RefCell;

use std::collections::HashMap;

use crate::icon;

pub struct Gui {
    pub plugin: XfcePanelPlugin,
    pub save_location: Option<String>,
    pub window: gtk::Window,
    pub event_box: gtk::EventBox,
    pub button: gtk::Button,
    pub main_container: gtk::Box,
    pub image: gtk::Image,
    pub label: gtk::Label,
    pub icon_size: usize,
    pub icons: HashMap<String, gdk_pixbuf::Pixbuf>,
    pub item_list_popup: Option<gtk::Dialog>
}


impl Gui {
    pub fn new (pointer: XfcePanelPluginPointer) -> Self {
        let mut plugin = XfcePanelPlugin::from(pointer);

        let save_location = plugin.save_location(true);

        let plugin_container = plugin.container.clone();

        let window: gtk::Window = plugin_container.get_parent().unwrap().downcast().unwrap();

        let event_box = gtk::EventBox::new();
        event_box.show();
        plugin_container.add(&event_box);


        let main_container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        main_container.show();
        event_box.add(&main_container);


        let button = gtk::Button::new();
        button.show();

        let image = gtk::ImageBuilder::new()
            .name("toolbar-image")
            .icon_name("rssplugin")
            .build();
        image.show();
        
        button.set_always_show_image(true);
        button.set_image(Some(&image));

        let label = gtk::Label::new(Some(""));
        label.show();
        main_container.add(&label);
        main_container.add(&button);

        plugin_container.show_all();
        plugin.show_about();
        plugin.show_configure();

        let icons: HashMap<String, gdk_pixbuf::Pixbuf> = HashMap::new();

        let mut gui = Gui {
            plugin,
            save_location,
            window,
            event_box,
            button,
            main_container,
            image,
            label,
            icon_size: 30,
            icons,
            item_list_popup: None,
        };

        gui.recreate_icons_in_size(gui.icon_size);

        gui
    }

    pub fn recreate_icons_in_size(&mut self, icon_size: usize) {
        self.icons.insert("alert".to_string(), icon::icon_by_color_and_size("#900", icon_size).unwrap());
        self.icons.insert("idle".to_string(), icon::icon_by_color_and_size("#CCC", icon_size).unwrap());
        self.icons.insert("inactive".to_string(), icon::icon_by_color_and_size("rgba(255,255,255,0.2)", icon_size).unwrap());
        self.icons.insert("notify".to_string(), icon::icon_by_color_and_size("#F60", icon_size).unwrap());
        self.icon_size = icon_size;
    }

    pub fn start(&self) {
        self.plugin.container.show_all();
    }


    pub fn render(&self, state: &State) {
        if state.orientation != self.main_container.get_orientation() {
            self.main_container.set_orientation(state.orientation);
        }
        self.event_box.set_tooltip_text(Some(&state.config.feed));
        if let Some(ref err) = state.error {
            self.image.set_from_pixbuf(Some(&self.icons.get("alert").unwrap()));
            match err {
                StateError::InvalidFeedUrl => {
                    self.button.set_tooltip_text(Some("Bad feed"));
                },
                StateError::CouldNotGetChannel => {
                    self.button.set_tooltip_text(Some("Error"));
                },
                StateError::CouldNotSave => {
                    self.button.set_tooltip_text(Some("Could not save"));
                }
            }
        } else if !state.config.active {
            self.image.set_from_pixbuf(Some(&self.icons.get("inactive").unwrap()));
            self.button.set_tooltip_text(Some("Inactive feed"));
        } else {
            let seen = state.seen_ids.len();
            let ids = state.ids.len();
            self.button.set_tooltip_text(Some(&format!("{} unread out of {} items.", ids-seen, ids)));
            if ids == seen {
                self.image.set_from_pixbuf(Some(&self.icons.get("idle").unwrap()));
            } else {
                self.image.set_from_pixbuf(Some(&self.icons.get("notify").unwrap()));
            }
        }
    }

    pub fn list_items(&mut self, state: &mut State){
        let item_list_popup = gtk::DialogBuilder::new()
            .decorated(false)
            .resizable(false)
            .transient_for(&self.window)
            .width_request(200)
            .height_request(400)
            .build();

        item_list_popup.set_position(gtk::WindowPosition::Mouse);

        let item_list_popup_scroll_box = gtk::ScrolledWindowBuilder::new()
            .vexpand(true)
            .hexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        item_list_popup_scroll_box.show();
        // item_list_popup.get_content_area().set_preferred_width(100);
        // item_list_popup.get_content_area().show();
        item_list_popup.get_content_area().add(&item_list_popup_scroll_box.clone());

        let item_list_popup_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        item_list_popup_box.show();
        item_list_popup_scroll_box.add(&item_list_popup_box);
        
        let container = item_list_popup_box.clone();
        state.items.iter().for_each(move |item| {
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
            container.add(&item_widget);
        });



        if let Some(dialog) = self.item_list_popup.clone() {
            dialog.destroy();
        }
        item_list_popup.show();
        self.item_list_popup = Some(item_list_popup);
    }

    pub fn about(&self){
        let about_dialog = gtk::AboutDialogBuilder::new()
            // .parent(self.window)
            .transient_for(&self.window)
            .logo_icon_name("rssplugin")
            .version("0.0.0")
            .program_name("RSS Plugin")
            .build();
        about_dialog.connect_response(|dialog, _response|{
            dialog.destroy();
        });
        about_dialog.show();
    }

    pub fn configure(gui: Arc<RefCell<Gui>>, state: Arc<RefCell<State>>){
        let mut config_dialog;
        let mut active;
        let mut feed;
        {
            let state = state.clone();
            let gui = gui.clone();
            config_dialog = gtk::Dialog::new_with_buttons(
                Some("Rss Panel Plugin"),
                Some(&gui.clone().borrow().window),
                gtk::DialogFlags::DESTROY_WITH_PARENT,
                &[
                    ("gtk-close", gtk::ResponseType::Close),
                    ("gtk-save", gtk::ResponseType::Accept)
                ]
            );
            config_dialog.set_position(gtk::WindowPosition::Center);
            config_dialog.set_icon_name(Some("xfce4-settings"));

            let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
            gtk::WidgetExt::set_name(&container, "config-form");
            container.show();

            let location_label = gtk::Label::new(Some("No save location found. Config might not be preserved"));
            if let Some(loc) = gui.clone().borrow().save_location.clone() {
                location_label.set_label(&loc);
            }
            location_label.show();
            container.add(&location_label);

            let feed_label = gtk::LabelBuilder::new()
                .label("RSS Feed:")
                .build();
            feed_label.show();
            container.add(&feed_label);

            feed = gtk::EntryBuilder::new()
                .placeholder_text("https://...")
                .name("feed")
                .build();
            feed.show();
            container.add(&feed);
            
            active = gtk::CheckButtonBuilder::new()
                .label("Active")
                .name("active")
                .build();
            active.show();
            container.add(&active);
                
            config_dialog.get_content_area().add(&container);

            feed.set_text(&state.borrow().config.feed);
            active.set_active(state.borrow().config.active);

            config_dialog.show();
        }
    
        {
            let state = state.clone();
            let gui = gui.clone();
            config_dialog.connect_response(move |dialog, response|{
                let mut state = state.borrow_mut();
                let gui = gui.borrow();
                let save_location = gui.save_location.clone();
                let mut config = state.config.clone();
                match response {
                    gtk::ResponseType::Accept => {
                        config.active = active.get_active();
                        if let Some(value) = feed.get_text() {
                            config.feed = String::from(value.as_str());
                            if config.feed.len() < 5 {
                                state.error = Some(StateError::InvalidFeedUrl);
                                config.active = false;
                            }
                        };
                        if let Some(rc_file) = save_location {
                            match config.save(rc_file) {
                                Err(_) => {
                                    state.error = Some(StateError::CouldNotSave);
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
                state.config = config;
                gui.render(&state);
                dialog.destroy();
            });
        }
    }
}