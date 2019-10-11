use crate::xfce::ffi::{XfceScreenPosition, XfceSize};
use std::fmt;

pub enum ErrorType {
    InvalidFeedUrl,
    CouldNotGetChannel,
    // CouldNotSave,
    CouldNotReadConfigFile
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::InvalidFeedUrl => {
                write!(f, "{}", "Bad feed")
            },
            ErrorType::CouldNotGetChannel => {
                write!(f, "{}", "Error")
            },
            // ErrorType::CouldNotSave => {
            //     write!(f, "{}", "Could not save")
            // }
            ErrorType::CouldNotReadConfigFile => {
                write!(f, "{}", "Could not read config file")
            }
        }
    }
}


pub struct State {
    pub error: Option<ErrorType>,
    pub orientation: gtk::Orientation,
    pub position: XfceScreenPosition,
    pub size: XfceSize,
    pub seen_ids: Vec<rss::Guid>,
    pub ids: Vec<rss::Guid>,
    pub items: Vec<rss::Item>,
    pub show_items: bool,
    pub is_polling: bool
}

impl State {
    pub fn new () -> Self {
        State {
            error: None,
            orientation: gtk::Orientation::Horizontal,
            position: XfceScreenPosition::None,
            size: 0,
            seen_ids: Vec::new(),
            ids: Vec::new(),
            items: Vec::new(),
            show_items: false,
            is_polling: false
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
