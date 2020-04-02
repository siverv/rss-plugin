extern crate libc;
extern crate gtk;
extern crate gtk_sys;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gdk_sys;
extern crate rss;
extern crate reqwest;
#[macro_use] extern crate shrinkwraprs;

mod res;
mod xfce;
mod state;
mod gui;
mod config;
mod feed;
mod ui;
mod app;

use crate::xfce::ffi::XfcePanelPluginPointer;


#[no_mangle]
pub extern fn constructor(pointer: XfcePanelPluginPointer) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    app::App::start(pointer);
}
