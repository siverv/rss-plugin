
use std::sync::Arc;
use std::cell::{RefCell};

use gtk::{DialogExt,WidgetExt,ToggleButtonExt,EntryExt};

use crate::app::App;

use crate::xfce::rc::*;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub active: bool,
    pub feed: String,
    pub polling_interval: u32,
    pub save_location: Option<String>
}


impl Config {
    pub fn new(rc_file: &Option<String>) -> Result<Self,Box<std::error::Error>> {
        if let Some(file) = rc_file.clone() {
            Ok(Config::from(file))
        } else {
            Ok(Config::default())
        }
    }
    pub fn from(rc_file: String) -> Self {
        let mut config = rc_simple(&rc_file, |rc| {
            let mut config = Config::default();
            config.feed = read_entry(rc, "feed", "").clone();
            config.active = read_bool_entry(rc, "active", false);
            config.polling_interval = std::convert::TryInto::try_into(read_int_entry(rc, "polling_interval", 60 * 1000)).unwrap();
            config
        });
        config.save_location = Some(rc_file);
        config
    }

    pub fn save(&self, rc_file: String) {
        let config = self.clone();
        rc_simple_mut(&rc_file, |rc| {
            write_entry(rc, "feed", &config.feed);
            write_bool_entry(rc, "active", config.active);
            write_int_entry(rc, "polling_interval", std::convert::TryInto::try_into(config.polling_interval).unwrap());
        });
    }

    pub fn connect_save_if_confirmed(app_ref: Arc<RefCell<App>>, widgets: (gtk::Dialog, gtk::Entry, gtk::CheckButton)) {
        let (dialog, feed, active) = widgets.clone();
        dialog.connect_response(move |dialog, response|{
            if response == gtk::ResponseType::Accept {
                app_ref.borrow_mut().update(|_, mut state, mut config| {
                    config.active = active.get_active();
                    if let Some(value) = feed.get_text() {
                        config.feed = String::from(value.as_str());
                        if config.feed.len() < 5 {
                            state.error = Some(crate::state::ErrorType::InvalidFeedUrl);
                            config.active = false;
                        }
                    };
                    if let Some(rc_file) = config.save_location.clone() {
                        config.save(rc_file);
                    }
                    
                });
                dialog.destroy();
                crate::feed::start_polling_feed(app_ref.clone());
            }
        });
    }
}
