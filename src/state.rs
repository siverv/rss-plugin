use crate::xfce::plugin::*;
use crate::config::Config;

pub enum ErrorType {
    InvalidFeedUrl,
    CouldNotGetChannel,
    CouldNotSave
}

pub struct State {
    pub error: Option<ErrorType>,
    pub orientation: gtk::Orientation,
    pub position: XfceScreenPosition,
    pub size: XfceSize,
    pub config: Config,
    pub seen_ids: Vec<rss::Guid>,
    pub ids: Vec<rss::Guid>,
    pub items: Vec<rss::Item>,
    pub show_items: bool
}

impl State {
    pub fn new (rc_file: Option<String>) -> Self {
        State {
            error: None,
            orientation: gtk::Orientation::Horizontal,
            position: XfceScreenPosition::None,
            size: 0,
            config: Config::new(rc_file),
            seen_ids: Vec::new(),
            ids: Vec::new(),
            items: Vec::new(),
            show_items: false
        }
    }
    pub fn set_orientation (&mut self, orientation: gtk_sys::GtkOrientation) {
        match orientation {
            gtk_sys::GTK_ORIENTATION_VERTICAL => {
                self.orientation = gtk::Orientation::Vertical;
            },
            _ => {
                self.orientation = gtk::Orientation::Horizontal;
            }
        }
    }
}