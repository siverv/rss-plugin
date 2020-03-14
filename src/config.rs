
use gtk::{WidgetExt};

use crate::app::{App, AppEvent};
use crate::state::{StateEvent};
use crate::feed::{FeedEvent};
use crate::ui::config_dialog::ConfigDialog;

use crate::xfce::rc::*;

pub enum ConfigEvent {
    DialogResponse(gtk::ResponseType),
    Loaded(bool),
    // Save,
    Saved(bool)

}

#[derive(Debug, Clone)]
pub struct Config {
    loaded: bool,
    pub active: bool,
    pub preserve_items: bool,
    pub feed: String,
    pub polling_interval: u32,
    pub save_location: Option<String>,
}

impl Default for Config {
    fn default () -> Self {
        Config {
            loaded: false,
            active: false,
            preserve_items: false,
            feed: String::default(),
            polling_interval: u32::default(),
            save_location: None
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn reducer(app: &mut App, event: AppEvent) {
        if let AppEvent::ConfigEvent(event) = event {
            match event {
                ConfigEvent::DialogResponse(gtk::ResponseType::Accept) => {
                    Config::save_and_close_config_dialog(app);
                }
                _ => {}
            }
        }
    }
    
    pub fn init(app: &mut App) {
        app.config.save_location = app.gui.plugin.save_location(true);
        Config::load(app);
    }

    pub fn load(app: &mut App) {
        if let Some(rc_file) = &app.config.save_location {
            let (feed, active, interval, preserve_items) = rc_simple(rc_file, |rc| {
                (
                    read_entry(rc, "feed", "").clone(),
                    read_bool_entry(rc, "active", false),
                    std::convert::TryInto::try_into(read_int_entry(rc, "polling_interval", 60 * 1000)).unwrap(),
                    read_bool_entry(rc, "preserve_items", false),
                )
            });
            app.config.feed = feed;
            app.config.active = active;
            app.config.preserve_items = preserve_items;
            app.config.polling_interval = interval;
            app.dispatch(AppEvent::ConfigEvent(
                ConfigEvent::Loaded(true)
            ));
        } else {
            app.dispatch(AppEvent::ConfigEvent(
                ConfigEvent::Loaded(false)
            ));
        }
    }

    pub fn save(app: &mut App) -> bool {
        if let Some(rc_file) = &app.config.save_location {
            rc_simple_mut(rc_file, |rc| {
                write_entry(rc, "feed", &app.config.feed);
                write_bool_entry(rc, "active", app.config.active);
                write_int_entry(rc, "polling_interval", std::convert::TryInto::try_into(app.config.polling_interval).unwrap());
                write_bool_entry(rc, "preserve_items", app.config.preserve_items);
            });
            app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Saved(true)));
            return true;
        } else {
            app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Saved(false)));
            return false;
        }
    }

    pub fn set_from_dialog(&mut self, dialog: &ConfigDialog) -> Result<(),crate::state::ErrorType> {
        self.active = dialog.get_active();
        self.preserve_items = dialog.get_preserve_items();
        let feed = dialog.get_feed();
        // TODO: If new feed, dispatch FeedEvent::Clear
        if is_feed_valid(&feed) {
            self.feed = feed;
        } else {
            self.active = false;
            return Err(crate::state::ErrorType::InvalidFeedUrl);
        }
        return Ok(());
    }

    fn save_and_close_config_dialog(app: &mut App){
        if let Some(dialog) = &app.gui.config_dialog {
            if let Err(error) = app.config.set_from_dialog(&dialog) {
                app.dispatch(AppEvent::StateEvent(
                    StateEvent::Error(error)
                ));
                return;
            }
            let ok = Config::save(app);
            if !ok {
                // TODO: Handle
            }
            Config::close_config_dialog(app);
            if app.config.active {
                app.dispatch(AppEvent::FeedEvent(FeedEvent::Start))
            }
        } else {
            // TODO: Error
            return;
        } 
    }

    fn close_config_dialog(app: &mut App) {
        if let Some(dialog) = &app.gui.config_dialog {
            dialog.destroy();
        }
    }
}

fn is_feed_valid(feed: &str) -> bool {
    return feed.len() >= 5;
}
