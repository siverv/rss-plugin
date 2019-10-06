extern crate libc;
extern crate gtk;
extern crate gtk_sys;
extern crate glib;
extern crate rss;
use gtk::prelude::*;

use std::convert::TryInto;


use std;
use std::sync::Arc;
use std::cell::RefCell;

mod icon;
mod xfce;
use xfce::plugin::*;
mod state;
use state::State;
mod gui;
use gui::Gui;
mod config;

fn start_polling_feed(gui: Arc<RefCell<Gui>>, state: Arc<RefCell<State>>) {
    let mut poll_fn;
    let mut timeout = 60 * 1000;
    {
        let state = state.clone();
        let gui = gui.clone();
        let config = state.borrow().config.clone();

        if !config.active {
            return;
        }

        timeout = std::cmp::max(timeout, config.polling_interval);
        let feed_url = config.feed;
        if let None = state.clone().borrow().error {
            poll_fn =  Some(move || {
                let mut state = state.borrow_mut();
                let config = state.config.clone();
                if !config.active {
                    return Continue(false);
                }
                let channel = rss::Channel::from_url(&feed_url);
                    // "https://en.wikipedia.org/w/index.php?title=Special:NewPages&feed=rss"
                if let Ok(channel) = channel {
                    let ids = channel.items().iter().map(|item| item.guid());
                    let ids: Vec<rss::Guid> = ids.map(|x| x.unwrap().clone()).collect();
                    let seen_ids = state.seen_ids.clone();
                    let seen_ids: Vec<rss::Guid> = ids.iter().filter(|id| seen_ids.contains(id)).map(|id| id.clone()).collect();
                    state.ids = ids;
                    state.seen_ids = seen_ids;
                    state.items = channel.items().iter().map(|item| item.clone()).collect();
                } else {
                    state.error = Some(state::ErrorType::CouldNotGetChannel);
                    return gtk::Continue(false);
                }
                gui.borrow().render(&state);
                gtk::Continue(true)
            });
        } else {
            poll_fn = None
        }
    }
    if let Some(callback) = poll_fn {
        callback();
        gtk::timeout_add(timeout, callback);
    }
}


#[no_mangle]
pub extern fn constructor(pointer: XfcePanelPluginPointer) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }       
    let gui = Arc::new(RefCell::new(Gui::new(pointer)));
    let save_location = gui.borrow().save_location.clone();
    let state = Arc::new(RefCell::new(State::new(save_location)));

    {
        let state = state.clone();
        let gui = gui.clone();
        gui.clone().borrow().plugin.connect_orientation_changed(move |orientation| {
            let mut state = state.borrow_mut();
            state.set_orientation(orientation);
            gui.borrow().render(&state);
        });
    }
    {
        let state = state.clone();
        let gui = gui.clone();
        gui.clone().borrow().plugin.connect_screen_position_changed(move |screen_position| {
            let mut state = state.borrow_mut();
            state.position = screen_position;
            gui.borrow().render(&state);
        });
    }
    {
        let state = state.clone();
        let gui = gui.clone();
        gui.clone().borrow().plugin.connect_size_changed(move |size| {
            let mut state = state.borrow_mut();
            state.size = size;
            let mut gui = gui.borrow_mut();
            gui.icon_size = size.try_into().unwrap();
            gui.recreate_icons_in_size(size.try_into().unwrap());
            gui.render(&state);
        });
    }
    {
        let gui = gui.clone();
        gui.clone().borrow().plugin.connect_about(move || {
            gui.borrow().about();
        });
    }
    {
        let state = state.clone();
        let gui = gui.clone();
        gui.clone().borrow().plugin.connect_configure(move || {
            Gui::configure(gui.clone(), state.clone());
        });
    }
  
    {
        let state = state.clone();
        let gui = gui.clone();
        gui.clone().borrow().button.connect_clicked(move |_button| {
            let mut state = state.borrow_mut();
            state.seen_ids = state.ids.clone();
            let mut gui2 = gui.borrow_mut();
            if let Some(dialog) = gui2.item_list_popup.clone() {
                dialog.destroy();
                gui2.item_list_popup = None;
            } else {
                gui2.list_items(&mut state);
                let item_list_popup = gui2.item_list_popup.clone();
                let gui3 = gui.clone();
                item_list_popup.unwrap().connect_close(move |_dialog| {
                    gui3.borrow_mut().item_list_popup = None;
                });
            }

            gui2.render(&state);
        });
    }


    start_polling_feed(gui.clone(), state.clone());

    gui.borrow().start();
}

