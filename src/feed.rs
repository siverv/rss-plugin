
use crate::app::App;
use gtk::Continue;



use std::sync::Arc;
use std::cell::{RefCell};



const MINIMUM_TIMEOUT: u32 = 60 * 1000;

pub fn stop_polling(app_ref: Arc<RefCell<App>>) -> Continue {
    app_ref.borrow_mut().update(|_, mut state, _| {
        state.is_polling = false;
    });
    Continue(false)
}


pub fn start_polling_feed(app_ref: Arc<RefCell<App>>) {
    let mut poll_fn;
    let mut timeout = MINIMUM_TIMEOUT;
    {
        if !app_ref.borrow().config.active || app_ref.borrow().state.is_polling {
            return;
        }

        timeout = std::cmp::max(timeout, app_ref.borrow().config.polling_interval);
        
        if let None = app_ref.borrow().state.error {
            let app_ref = app_ref.clone();
            poll_fn = Some(move || {
                if !app_ref.borrow().config.active {
                    return stop_polling(app_ref.clone());
                }
                let feed_url = app_ref.borrow().config.feed.clone();
                let channel = rss::Channel::from_url(&feed_url);
                if let Ok(channel) = channel {
                    let ids = channel.items().iter().map(|item| item.guid());
                    let ids: Vec<rss::Guid> = ids.map(|x| x.unwrap().clone()).collect();
                    app_ref.borrow_mut().update(|_, mut state, _| {
                        let seen_ids = state.seen_ids.clone();
                        let seen_ids: Vec<rss::Guid> = ids.iter().filter(|id| seen_ids.contains(id)).map(|id| id.clone()).collect();
                        // https://en.wikipedia.org/w/index.php?title=Special:NewPages&feed=rss
                        // https://en.wikipedia.org/w/api.php?hidebots=1&hidecategorization=1&hideWikibase=1&urlversion=1&days=7&limit=50&action=feedrecentchanges&feedformat=rss
                        state.ids = ids;
                        state.seen_ids = seen_ids;
                        state.items = channel.items().iter().map(|item| item.clone()).collect();
                    });
                } else {
                    app_ref.borrow_mut().update(|_, mut state, _| {
                        state.error = Some(crate::state::ErrorType::CouldNotGetChannel);
                    });
                    return stop_polling(app_ref.clone());
                }
                gtk::Continue(true)
            });
        } else {
            poll_fn = None
        }
    }
    if let Some(callback) = poll_fn {
        {
            let callback = callback.clone();
            gtk::timeout_add(1000, move ||{
                callback();
                Continue(false)
            });
        }
        gtk::timeout_add(timeout, callback);
    }
}
