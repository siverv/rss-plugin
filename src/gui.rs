
use gtk::prelude::*;
use crate::xfce::{plugin::XfcePanelPlugin, ffi::*};
use std::convert::TryInto;

use crate::app::{App, AppEvent};
use crate::state::{StateEvent};
use crate::config::{ConfigEvent};
use crate::feed::{FeedEvent};

use crate::ui;
use crate::res::icon;

pub enum Dialog {
    About,
    Config,
    Feed
}

pub enum GuiEvent {
    Open(Dialog),
    Destroyed(Dialog),
    Toggle(Dialog)
    // Close(Dialog)
}

pub struct ConfigGui {
    pub dialog: gtk::Dialog,
    pub feed: gtk::Entry,
    pub active: gtk::CheckButton
}

pub struct FeedGui {
    pub dialog: gtk::Dialog
}

pub struct Gui {
    tx: glib::Sender<AppEvent>,
    pub plugin: XfcePanelPlugin,
    pub window: gtk::Window,
    pub event_box: gtk::EventBox,
    pub button: gtk::Button,
    pub main_container: gtk::Box,
    pub image: gtk::Image,
    pub label: gtk::Label,
    pub icons: icon::IconSet,
    pub item_list_popup: Option<gtk::Dialog>,
    pub config: Option<ConfigGui>,
    pub feed_dialog: Option<FeedGui>
}


impl Gui {
    pub fn new (pointer: XfcePanelPluginPointer, tx: glib::Sender<AppEvent>) -> Self {
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

        let gui = Gui {
            tx,
            plugin,
            window,
            event_box,
            button,
            main_container,
            image,
            label,
            icons: icon::IconSet::new(30),
            item_list_popup: None,
            config: None,
            feed_dialog: None
        };

        gui.connect_plugin();

        return gui;
    }


    pub fn reducer(app: &mut App, event: AppEvent) {
        if let AppEvent::GuiEvent(event) = event {
            match event {
                GuiEvent::Open(Dialog::About) => {
                    Gui::open_about(app);
                }
                GuiEvent::Open(Dialog::Config) => {
                    Gui::open_configure(app);
                }
                // GuiEvent::Open(Dialog::Feed) => {
                //     Gui::open_feed(app);
                // }
                GuiEvent::Destroyed(Dialog::About) => {
                    Gui::cleanup_about(app);
                }
                GuiEvent::Destroyed(Dialog::Config) => {
                    Gui::cleanup_configure(app);
                }
                GuiEvent::Destroyed(Dialog::Feed) => {
                    Gui::cleanup_feed(app);
                }
                GuiEvent::Toggle(Dialog::Feed) => {
                    Gui::toggle_feed(app);
                }
                _ => {}
            };
        }
    }

    pub fn init(app: &mut App) {
        app.gui.plugin.container.show_all();
    }

    pub fn connect_plugin(&self){
        {
            let tx = self.tx.clone();
            self.plugin.connect_orientation_changed(move |orientation| {
                tx.send(AppEvent::StateEvent(
                    StateEvent::SetOrientation(match orientation {
                        gtk_sys::GTK_ORIENTATION_VERTICAL => gtk::Orientation::Vertical,
                        _ => gtk::Orientation::Horizontal
                    })
                )).unwrap();
            });
        }
        {
            let tx = self.tx.clone();
            self.plugin.connect_screen_position_changed(move |position| {
                tx.send(AppEvent::StateEvent(
                    StateEvent::SetPosition(position)
                )).unwrap();
            });
        }
        {
            let tx = self.tx.clone();
            self.plugin.connect_size_changed(move |size| {
                tx.send(AppEvent::StateEvent(
                    StateEvent::SetSize(size)
                )).unwrap();
            });
        }
        {
            let tx = self.tx.clone();
            self.plugin.connect_about(move || {
                tx.send(AppEvent::GuiEvent(GuiEvent::Open(Dialog::About))).unwrap();
            });
        }
        {
            let tx = self.tx.clone();
            self.plugin.connect_configure(move || {
                tx.send(AppEvent::GuiEvent(GuiEvent::Open(Dialog::Config))).unwrap();
            });
        }
        {
            let tx = self.tx.clone();
            self.button.connect_clicked(move |_button| {
                tx.send(AppEvent::GuiEvent(GuiEvent::Toggle(Dialog::Feed))).unwrap();
            });
        }
    }

    pub fn recreate_icons(app: &mut App) {
        let icon_size = app.state.size.try_into().unwrap();
        if app.gui.icons.size != icon_size {
            app.gui.icons = icon::IconSet::new(icon_size);
        }
    }


    pub fn update(app: &App) {
        let config = &app.config;
        let state = &app.state;
        let feed = &app.feed;
        let gui = &app.gui;
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
            let unseen = feed.unseen_ids.len();
            let ids = feed.all_ids.len();
            gui.button.set_tooltip_text(Some(&format!("{} unread out of {} items.", unseen, ids)));
            if unseen == 0 {
                gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Idle)));
            } else {
                gui.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Notify)));
                if let Some(dialog) = gui.item_list_popup.clone() {
                    let (scrolling_area, container) = ui::feed_dialog::new_feed_list();
                    feed.items.iter().for_each(|item| {
                        container.add(&ui::feed_dialog::new_feed_item(item));
                    });
                    dialog.get_content_area().get_children().iter().for_each(|child| child.destroy());
                    dialog.get_content_area().add(&scrolling_area);
                    dialog.show_all();
                }
            }
        }
    }

    pub fn toggle_feed (app: &mut App) {
        if let None = app.gui.feed_dialog {
            Gui::open_feed(app);
        } else {
            Gui::close_feed(app);
        }
    }

    pub fn open_feed(app: &mut App) {
        let (dialog, container) = ui::feed_dialog::open_feed_dialog(&app.gui.window);
        dialog.show();
        app.feed.items.iter().for_each(|item| {
            container.add(&ui::feed_dialog::new_feed_item(item));
        });
        dialog.show_all();
        dialog.grab_focus();
        {
            let tx = app.tx.clone();
            dialog.connect_destroy(move |_| {
                tx.send(AppEvent::GuiEvent(GuiEvent::Destroyed(Dialog::Feed))).unwrap();
            });
        }
        app.dispatch(AppEvent::FeedEvent(FeedEvent::MarkAllAsSeen));
        app.gui.feed_dialog = Some(FeedGui{dialog});
    }

    pub fn close_feed(app: &mut App) {
        if let Some(FeedGui{dialog}) = &app.gui.feed_dialog {
            dialog.destroy();
            Gui::cleanup_feed(app);
        }
    }

    pub fn cleanup_feed(app: &mut App) {
        app.gui.feed_dialog = None;
    }

    pub fn open_about(app: &mut App){
        ui::about_dialog::open_about_dialog(&app.gui.window);
    }

    pub fn cleanup_about(_app: &mut App) {
        // NOOP;
    }

    pub fn open_configure(app: &mut App) {
        let (dialog, container) = ui::config_dialog::open_config_dialog(&app.gui.window);
        let location_label = ui::config_dialog::new_header(&app.config.save_location);
        let (feed_row, feed_entry, _) = ui::config_dialog::new_feed_entry();
        let active = ui::config_dialog::new_active_field();

        container.add(&location_label);
        container.add(&feed_row);
        container.add(&active);

        feed_entry.set_text(&app.config.feed);
        active.set_active(app.config.active);
        
        {
            let tx = app.tx.clone();
            dialog.connect_response(move |_dialog, response|{
                tx.send(AppEvent::ConfigEvent(ConfigEvent::DialogResponse(response))).unwrap();
            });
        }
        {
            let tx = app.tx.clone();
            dialog.connect_destroy(move |_| {
                tx.send(AppEvent::GuiEvent(GuiEvent::Destroyed(Dialog::Config))).unwrap();
            });
        }
        
        dialog.show();
        app.gui.config = Some(ConfigGui {
            dialog,
            feed: feed_entry,
            active
        });

        // app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Load));
    }

    pub fn cleanup_configure(app: &mut App) {
        app.gui.config = None;
    }

}
