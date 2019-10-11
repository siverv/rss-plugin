
use std::boxed::Box as Box_;
use gtk::prelude::*;
use glib::signal::connect_raw;
use libc::{c_int};
use glib::translate::*;
use glib::translate::{FromGlibPtrFull};
use std::mem::transmute;
use crate::xfce::ffi::*;


pub struct XfcePanelPlugin {
    pointer: * mut gtk_sys::GtkWidget,
    pub container: gtk::Container,
    pub orientation: gtk_sys::GtkOrientation,
    pub screen_position: XfceScreenPosition,
    pub size: c_int
}

#[allow(dead_code)]
impl XfcePanelPlugin {
    pub fn from(pointer: XfcePanelPluginPointer) -> Self {
        unsafe {
            let orientation = xfce_panel_plugin_get_orientation(pointer);
            let screen_position = xfce_panel_plugin_get_screen_position(pointer);
            let size = xfce_panel_plugin_get_size(pointer);
            XfcePanelPlugin {
                pointer,
                orientation,
                screen_position,
                size,
                container: gtk::Widget::from_glib_none(pointer).unsafe_cast()
            }
        }
    }

    pub fn noarg_connect<F: Fn() + 'static>(&self, signal: &'static str, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn trampoline_noarg<F: Fn() + 'static>(
            _this: *mut gtk_sys::GtkWidget,
            f: glib_sys::gpointer
         ) {
            let f: &F = &*(f as *const F);
            f();
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.container.as_ptr() as *mut _,
                signal.to_glib_none().0,
                Some(transmute(trampoline_noarg::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
    pub fn onearg_connect<T, F: Fn(T) + 'static>(&self, signal: &'static str, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn trampoline_onearg<T, F: Fn(T) + 'static>(
            _this: *mut gtk_sys::GtkWidget,
            t: T,
            f: glib_sys::gpointer
        ) {
            let f: &F = &*(f as *const F);
            f(t);
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.container.as_ptr() as *mut _,
                signal.to_glib_none().0,
                Some(transmute(trampoline_onearg::<T, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_orientation_changed<F: Fn(gtk_sys::GtkOrientation) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.onearg_connect("orientation-changed", f)
    }
    pub fn connect_screen_position_changed<F: Fn(XfceScreenPosition) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.onearg_connect("screen-position-changed", f)
    }
    pub fn connect_size_changed<F: Fn(c_int) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.onearg_connect("size-changed", f)
    }
    pub fn connect_free_data<F: Fn() + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.noarg_connect("free-data", f)
    }
    pub fn connect_save<F: Fn() + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.noarg_connect("save", f)
    }
    pub fn connect_about<F: Fn() + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.noarg_connect("about", f)
    }
    pub fn connect_configure<F: Fn() + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.noarg_connect("configure-plugin", f)
    }


    pub fn show_about(&mut self){
        unsafe {
            xfce_panel_plugin_menu_show_about(self.pointer);
        }
    }
    pub fn show_configure(&mut self){
        unsafe {
            xfce_panel_plugin_menu_show_configure(self.pointer);
        }
    }

    pub fn add_action_widget(&mut self, widget: &gtk::Widget){
        unsafe {
            xfce_panel_plugin_add_action_widget(self.pointer, widget.to_glib_none().0);
        }
    }

    pub fn menu_insert_item(&mut self, item: &gtk::MenuItem){
        unsafe {
            xfce_panel_plugin_menu_insert_item(self.pointer, item.to_glib_none().0);
        }
    }

    pub fn lookup_rc_file(&mut self) -> Option<String> {
        unsafe {
            let value = xfce_panel_plugin_lookup_rc_file(self.pointer);
            if value.is_null() {
                None
            } else {
                Some(FromGlibPtrFull::from_glib_full(value))
            }
        }
    }

    pub fn save_location(&self, create: bool) -> Option<String> {
        unsafe {
            let value = xfce_panel_plugin_save_location(self.pointer, create.to_glib());
            if value.is_null() {
                None
            } else {
                Some(FromGlibPtrFull::from_glib_full(value))
            }
        }
    }


}

pub enum XfceScreenPositionDirection { None, Floating, Top, Left, Right, Bottom }
pub enum XfceScreenPositionOrientation { None, Horizontal, Vertical }

#[allow(dead_code)]
impl XfceScreenPosition {
    pub fn get_direction(&self) -> XfceScreenPositionDirection {
        match self {
            XfceScreenPosition::None => XfceScreenPositionDirection::None,
            XfceScreenPosition::NorthWestHorizontal => XfceScreenPositionDirection::Top,
            XfceScreenPosition::North => XfceScreenPositionDirection::Top,
            XfceScreenPosition::NorthEastHorizontal => XfceScreenPositionDirection::Top,
            XfceScreenPosition::NorthWestVertical => XfceScreenPositionDirection::Left,
            XfceScreenPosition::West => XfceScreenPositionDirection::Left,
            XfceScreenPosition::SouthWestVertical => XfceScreenPositionDirection::Left,
            XfceScreenPosition::NorthEastVertical => XfceScreenPositionDirection::Right,
            XfceScreenPosition::East => XfceScreenPositionDirection::Right,
            XfceScreenPosition::SouthEastVertical => XfceScreenPositionDirection::Right,
            XfceScreenPosition::SouthWestHorizontal => XfceScreenPositionDirection::Bottom,
            XfceScreenPosition::South => XfceScreenPositionDirection::Bottom,
            XfceScreenPosition::SouthEastHorizontal => XfceScreenPositionDirection::Bottom,
            XfceScreenPosition::FloatingHorizontal => XfceScreenPositionDirection::Floating,
            XfceScreenPosition::FloatingVertical => XfceScreenPositionDirection::Floating
        }
    }
    pub fn get_orientation(&self) -> XfceScreenPositionOrientation {
        match self {
            XfceScreenPosition::None => XfceScreenPositionOrientation::None,
            XfceScreenPosition::NorthWestHorizontal => XfceScreenPositionOrientation::Horizontal,
            XfceScreenPosition::North => XfceScreenPositionOrientation::None,
            XfceScreenPosition::NorthEastHorizontal => XfceScreenPositionOrientation::Horizontal,
            XfceScreenPosition::NorthWestVertical => XfceScreenPositionOrientation::Vertical,
            XfceScreenPosition::West => XfceScreenPositionOrientation::None,
            XfceScreenPosition::SouthWestVertical => XfceScreenPositionOrientation::Vertical,
            XfceScreenPosition::NorthEastVertical => XfceScreenPositionOrientation::Vertical,
            XfceScreenPosition::East => XfceScreenPositionOrientation::None,
            XfceScreenPosition::SouthEastVertical => XfceScreenPositionOrientation::Vertical,
            XfceScreenPosition::SouthWestHorizontal => XfceScreenPositionOrientation::Horizontal,
            XfceScreenPosition::South => XfceScreenPositionOrientation::None,
            XfceScreenPosition::SouthEastHorizontal => XfceScreenPositionOrientation::Horizontal,
            XfceScreenPosition::FloatingHorizontal => XfceScreenPositionOrientation::Horizontal,
            XfceScreenPosition::FloatingVertical => XfceScreenPositionOrientation::Vertical
        }
    }
}
