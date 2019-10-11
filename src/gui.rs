
use gtk::prelude::*;
use gtk::OrientableExt;
use crate::xfce::{plugin::XfcePanelPlugin, ffi::*};

use crate::state::State;
use crate::config::Config;

use crate::ui;
use crate::res::icon;

pub struct Gui {
    pub plugin: XfcePanelPlugin,
    pub window: gtk::Window,
    pub event_box: gtk::EventBox,
    pub button: gtk::Button,
    pub main_container: gtk::Box,
    pub image: gtk::Image,
    pub label: gtk::Label,
    pub icons: icon::IconSet,
    pub item_list_popup: Option<gtk::Dialog>
}


impl Gui {
    pub fn new (pointer: XfcePanelPluginPointer) -> Self {
        let mut plugin = XfcePanelPlugin::from(pointer);

        let plugin_container = plugin.container.clone();

        let window: gtk::Window = plugin_container.get_parent().unwrap().downcast().unwrap();


        let (event_box, main_container, label) = ui::panel_box::new_panel_box();
        plugin_container.add(&event_box);

        let (button, image) = ui::panel_icon::new_panel_icon();
        main_container.add(&button);

        plugin_container.show_all();
        plugin.show_about();
        plugin.show_configure();

        Gui {
            plugin,
            window,
            event_box,
            button,
            main_container,
            image,
            label,
            icons: icon::IconSet::new(30),
            item_list_popup: None,
        }
    }

    pub fn recreate_icons_in_size(&mut self, icon_size: usize) {
        if self.icons.size != icon_size {
            self.icons = icon::IconSet::new(icon_size);
        }
    }

    pub fn start(&self) {
        self.plugin.container.show_all();
    }


    pub fn update(gui: &Gui, state: &State, config: &Config) {
        if state.orientation != gui.main_container.get_orientation() {
            gui.main_container.set_orientation(state.orientation);
        }
        gui.event_box.set_tooltip_text(Some(&config.feed));
        if let Some(ref err) = state.error {
            gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Alert)));
            gui.button.set_tooltip_text(Some(&format!("{}", err)));
        } else if !config.active {
            gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Inactive)));
            gui.button.set_tooltip_text(Some("Inactive feed"));
        } else {
            let seen = state.seen_ids.len();
            let ids = state.ids.len();
            gui.button.set_tooltip_text(Some(&format!("{} unread out of {} items.", ids-seen, ids)));
            if ids == seen {
                gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Idle)));
            } else {
                gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Notify)));
                if let Some(dialog) = gui.item_list_popup.clone() {
                    let (scrolling_area, container) = ui::feed_dialog::new_feed_list();
                    state.items.iter().for_each(|item| {
                        container.add(&ui::feed_dialog::new_feed_item(item));
                    });
                    dialog.get_content_area().get_children().iter().for_each(|child| child.destroy());
                    dialog.get_content_area().add(&scrolling_area);
                    dialog.show_all();
                }
            }
        }
    }

    pub fn open_feed(gui: &Gui, state: &State, _: &Config) -> gtk::Dialog {
        let mut feed_dialog;
        {
            let (dialog, container) = ui::feed_dialog::open_feed_dialog(&gui.window);
            dialog.show();
            state.items.iter().for_each(|item| {
                container.add(&ui::feed_dialog::new_feed_item(item));
            });
            feed_dialog = dialog;
        }
        feed_dialog.show_all();
        feed_dialog.grab_focus();
        feed_dialog
    }

    pub fn open_about(gui: &Gui, _: &State, _: &Config){
        ui::about_dialog::open_about_dialog(&gui.window);
    }

    pub fn open_configure(gui: &Gui, _state: &State, config: &Config)
        -> (gtk::Dialog, gtk::Entry, gtk::CheckButton) {
        let (dialog, container) = ui::config_dialog::open_config_dialog(&gui.window);
        let location_label = ui::config_dialog::new_header(&config.save_location);
        let (feed_row, feed_entry, _) = ui::config_dialog::new_feed_entry();
        let active = ui::config_dialog::new_active_field();

        container.add(&location_label);
        container.add(&feed_row);
        container.add(&active);

        feed_entry.set_text(&config.feed);
        active.set_active(config.active);
        dialog.show();
        
        (dialog, feed_entry, active)
    }
}
