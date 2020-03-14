
use gtk::prelude::*;
use crate::xfce::{plugin::XfcePanelPlugin, ffi::*};
use std::convert::TryInto;

use crate::app::{App, AppEvent};
use crate::state::{StateEvent};
use crate::config::{ConfigEvent};
use crate::feed::{FeedEvent};

use crate::ui;
use crate::ui::{
    about_dialog::AboutDialog,
    config_dialog::ConfigDialog,
    feed_dialog::FeedDialog
};
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

pub struct Gui {
    tx: glib::Sender<AppEvent>,
    pub plugin: XfcePanelPlugin,
    pub window: gtk::Window,
    pub icons: icon::IconSet,
    pub panel_box: ui::panel_box::PanelBox,
    pub panel_icon: ui::panel_icon::PanelIcon,
    pub about_dialog: Option<AboutDialog>,
    pub config_dialog: Option<ConfigDialog>,
    pub feed_dialog: Option<FeedDialog>
}


impl Gui {
    pub fn new (pointer: XfcePanelPluginPointer, tx: glib::Sender<AppEvent>) -> Self {
        let mut plugin = XfcePanelPlugin::from(pointer);

        let plugin_container = plugin.container.clone();

        let window: gtk::Window = plugin_container.get_parent().unwrap().downcast().unwrap();


        let panel_box = ui::panel_box::PanelBox::new();
        plugin_container.add(panel_box.as_widget());

        let panel_icon = ui::panel_icon::PanelIcon::new();
        panel_box.container.add(panel_icon.as_widget());

        plugin_container.show_all();
        plugin.show_about();
        plugin.show_configure();

        let gui = Gui {
            tx,
            plugin,
            window,
            panel_box,
            panel_icon,
            icons: icon::IconSet::new(30),
            config_dialog: None,
            about_dialog: None,
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
            self.panel_icon.connect_clicked(move |_button| {
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


    pub fn update(app: &mut App) {
        let config = &app.config;
        let state = &app.state;
        let feed = &app.feed;
        let gui = &mut app.gui;
        if state.orientation != gui.panel_box.container.get_orientation() {
            gui.panel_box.container.set_orientation(state.orientation);
        }
        gui.panel_box.set_tooltip_text(Some(&config.feed));
        if let Some(ref err) = state.error {
            gui.panel_icon.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Alert)));
            gui.panel_icon.set_tooltip_text(Some(&format!("{}", err)));
        } else if !config.active {
            gui.panel_icon.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Inactive)));
            gui.panel_icon.set_tooltip_text(Some("Inactive feed"));
        } else {
            let unseen = feed.unseen_ids.len();
            let ids = feed.all_ids.len();
            gui.panel_icon.set_tooltip_text(Some(&format!("{} unread out of {} items.", unseen, ids)));
            if unseen == 0 {
                gui.panel_icon.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Idle)));
            } else {
                gui.panel_icon.image.set_from_pixbuf(Some(&gui.icons.get(icon::IconType::Notify)));
                if let Some(dialog) = &mut gui.feed_dialog {
                    dialog.set_items(&feed.items);
                }
            }
        }
    }

    pub fn toggle_feed (app: &mut App) {
        if app.gui.feed_dialog.is_none() {
            Gui::open_feed(app);
        } else {
            Gui::close_feed(app);
        }
    }

    pub fn open_feed(app: &mut App) {
        let mut dialog = FeedDialog::new(&app.gui.window);
        dialog.show();
        {
            let tx = app.tx.clone();
            dialog.connect_destroy(move |_| {
                tx.send(AppEvent::GuiEvent(GuiEvent::Destroyed(Dialog::Feed))).unwrap();
            });
        }
        dialog.set_items(&app.feed.items);
        dialog.grab_focus();
        app.dispatch(AppEvent::FeedEvent(FeedEvent::MarkAllAsSeen));
        app.gui.feed_dialog = Some(dialog)
    }

    pub fn close_feed(app: &mut App) {
        if let Some(dialog) = &app.gui.feed_dialog {
            dialog.destroy();
            Gui::cleanup_feed(app);
        }
    }

    pub fn cleanup_feed(app: &mut App) {
        app.gui.feed_dialog = None;
    }

    pub fn open_about(app: &mut App){
        if app.gui.about_dialog.is_none() {
            let dialog = AboutDialog::new(&app.gui.window);
            {
                let tx = app.tx.clone();
                dialog.connect_destroy(move |_| {
                    tx.send(AppEvent::GuiEvent(GuiEvent::Destroyed(Dialog::About))).unwrap();
                });
            }
            app.gui.about_dialog = Some(dialog);
        }
    }

    pub fn cleanup_about(app: &mut App) {
        app.gui.about_dialog = None;
    }

    pub fn open_configure(app: &mut App) {
        let dialog = ConfigDialog::new(&app.gui.window);
        dialog.update(app);
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
        {
            let tx = app.tx.clone();
            dialog.connect_test_feed(move |_| {
                tx.send(AppEvent::FeedEvent(FeedEvent::Test)).unwrap();
            });
        }
        
        dialog.show();
        app.gui.config_dialog = Some(dialog);

        // app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Load));
    }

    pub fn cleanup_configure(app: &mut App) {
        app.gui.config_dialog = None;
    }

}
