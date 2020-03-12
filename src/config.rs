
use gtk::{WidgetExt,ToggleButtonExt,EntryExt};

use crate::app::{App, AppEvent};
use crate::state::{StateEvent};
use crate::gui::{ConfigGui};
use crate::feed::{FeedEvent};

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
    pub feed: String,
    pub polling_interval: u32,
    pub save_location: Option<String>,
}

impl Default for Config {
    fn default () -> Self {
        Config {
            loaded: false,
            active: false,
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
            let (feed, active, interval) = rc_simple(rc_file, |rc| {
                (
                    read_entry(rc, "feed", "").clone(),
                    read_bool_entry(rc, "active", false),
                    std::convert::TryInto::try_into(read_int_entry(rc, "polling_interval", 60 * 1000)).unwrap()
                )
            });
            app.config.feed = feed;
            app.config.active = active;
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
            });
            app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Saved(true)));
            return true;
        } else {
            app.dispatch(AppEvent::ConfigEvent(ConfigEvent::Saved(false)));
            return false;
        }
    }

    fn is_feed_valid(feed: &str) -> bool {
        return feed.len() >= 5;
    }

    pub fn set_and_validate_config_from_gui(app: &mut App) -> bool {
        if let Some(ConfigGui {dialog: _, active, feed}) = &app.gui.config {
            app.config.active = active.get_active();
            if let Some(value) = feed.get_text() {
                // TODO: If new feed, dispatch FeedEvent::Clear
                if Self::is_feed_valid(value.as_str()) {
                    app.config.feed = String::from(value.as_str());
                } else {
                    app.config.active = false;
                    app.dispatch(AppEvent::StateEvent(
                        StateEvent::Error(
                            crate::state::ErrorType::InvalidFeedUrl
                        )
                    ));
                }
            };
        }
        return true;
    }

    pub fn save_and_close_config_dialog(app: &mut App){
        if let None = app.gui.config {
            // TODO: Error
            return;
        }
        let valid = Config::set_and_validate_config_from_gui(app);
        if !valid {
            // TODO: Handle
        }
        let ok = Config::save(app);
        if !ok {
            // TODO: Handle
        }
        Config::close_config_dialog(app);
        if app.config.active {
            app.dispatch(AppEvent::FeedEvent(FeedEvent::Start))
        }
    }

    pub fn close_config_dialog(app: &mut App) {
        if let Some(config_gui) = &app.gui.config {
            config_gui.dialog.destroy();
        }
    }
}
