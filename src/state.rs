use crate::xfce::ffi::{XfceScreenPosition, XfceSize};
use std::fmt;
use crate::gui::{Gui};
use crate::app::{App, AppEvent};


pub enum ErrorType {
    InvalidFeedUrl,
    CouldNotGetChannel,
    // CouldNotSave,
    // CouldNotReadConfigFile,
    CouldNotDispatch
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
            // ErrorType::CouldNotReadConfigFile => {
            //     write!(f, "{}", "Could not read config file")
            // }
            ErrorType::CouldNotDispatch => {
                write!(f, "{}", "Could not dispatch state in app")
            }
        }
    }
}

pub enum StateEvent {
    Error(ErrorType),
    SetOrientation(gtk::Orientation),
    SetPosition(XfceScreenPosition),
    SetSize(XfceSize)
}

pub struct State {
    pub error: Option<ErrorType>,
    pub orientation: gtk::Orientation,
    pub position: XfceScreenPosition,
    pub size: XfceSize
}

impl State {
    pub fn new () -> Self {
        State {
            error: None,
            orientation: gtk::Orientation::Horizontal,
            position: XfceScreenPosition::None,
            size: 30,
        }
    }

    pub fn reducer (app: &mut App, event: AppEvent) {
        if let AppEvent::StateEvent(event) = event {
            match event {
                StateEvent::Error(error) => {
                    app.state.error = Some(error);
                }
                StateEvent::SetOrientation(orientation) => app.state.orientation = orientation,
                StateEvent::SetPosition(position) => app.state.position = position,
                StateEvent::SetSize(size) => {
                    app.state.size = size;
                    Gui::recreate_icons(app);
                }
            }
        }
    }
}


