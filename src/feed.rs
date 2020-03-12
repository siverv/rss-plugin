
use crate::app::{App, AppEvent};
use crate::state::{StateEvent};
use glib::SourceId;
use glib::translate::{FromGlib, ToGlib};

pub enum ReasonForStopping {
    // CleanStop,
    // Restart,
    FaultyStop
}

pub enum FeedEvent {
    Poll,
    Polled(Option<()>),
    Start,
    Started,
    // Stop,
    Stopped(ReasonForStopping),
    // Clear,
    MarkAllAsSeen
}

pub struct Feed {
    pub is_polling: bool,
    pub polling_id: Option<glib::SourceId>,
    pub all_ids: Vec<rss::Guid>,
    pub unseen_ids: Vec<rss::Guid>,
    pub new_ids: Vec<rss::Guid>,
    pub items: Vec<rss::Item>
}

const MINIMUM_TIMEOUT: u32 = 60 * 1000;

impl Feed {
    pub fn new () -> Self {
        return Feed {
            is_polling: false,
            polling_id: None,
            all_ids: Vec::default(),
            unseen_ids: Vec::default(),
            new_ids: Vec::default(),
            items: Vec::default()
        }
    }

    pub fn reducer(app: &mut App, event: AppEvent) {
        if let AppEvent::FeedEvent(event) = event {
            match event {
                FeedEvent::Poll => Feed::poll_feed(app),
                FeedEvent::Start => Feed::start_polling(app),
                // FeedEvent::Stop => Feed::stop_polling(app, ReasonForStopping::CleanStop),
                // FeedEvent::Clear => Feed::clear_feed(app),
                FeedEvent::MarkAllAsSeen => Feed::mark_all_as_seen(app),
                _ => {}
            }
        }
    }

    fn get_timeout(_app: &App) -> u32 {
        return MINIMUM_TIMEOUT;
    }

    fn start_polling(app: &mut App) {
        if app.feed.is_polling {
            // QUESTION: Trigger immediate polling?
            return;
        }
        app.feed.is_polling = true;
        let timeout = Feed::get_timeout(app);
        app.dispatch(AppEvent::FeedEvent(FeedEvent::Started));
        app.dispatch(AppEvent::FeedEvent(FeedEvent::Poll)); // QUESTION: Replace with immediate polling?
        {
            let tx = app.tx.clone();
            app.feed.polling_id = Some(gtk::timeout_add(timeout, move || {
                tx.send(AppEvent::FeedEvent(FeedEvent::Poll)).unwrap();
                gtk::Continue(true)
            }));
        }
    }

    fn stop_polling(app: &mut App, reason: ReasonForStopping) {
        if !app.feed.is_polling {
            return;
        }
        app.feed.is_polling = false;
        if let Some(source_id) = &app.feed.polling_id {
            // TODO: Fix hackish solution to borrowed SourceId.
            glib::source::source_remove(SourceId::from_glib(source_id.to_glib()));
        }
        app.feed.polling_id = None;
        app.dispatch(AppEvent::FeedEvent(FeedEvent::Stopped(reason)));
    }

    
    // fn restart_polling(app: &mut App) {
    //     Feed::stop_polling(app, ReasonForStopping::Restart);
    //     Feed::start_polling(app);
    // }

    // fn clear_feed(app: &mut App) {
    //     app.feed.all_ids = Vec::default();
    //     app.feed.unseen_ids = Vec::default();
    //     app.feed.new_ids = Vec::default();
    //     app.feed.items = Vec::default();
    // }

    fn mark_all_as_seen(app: &mut App) {
        app.feed.unseen_ids = Vec::new();
    }

    fn poll_feed(app: &mut App) {
        let feed_url = app.config.feed.clone();
        let channel = rss::Channel::from_url(&feed_url);
        if let Ok(channel) = channel {
            // QUESTION: Always clear out old, or keep appending new items?
            let mut new_items: Vec<rss::Item> = channel.items().iter().filter(|item| {
                !app.feed.all_ids.contains(item.guid().unwrap())
            }).map(|item| item.clone()).collect();
            new_items.iter().for_each(|item| {
                let id = item.guid().unwrap();
                app.feed.all_ids.push(id.clone());
                app.feed.unseen_ids.push(id.clone());
            });
            if new_items.len() > 0 {
                app.feed.items.append(&mut new_items);
                app.dispatch(AppEvent::FeedEvent(
                    FeedEvent::Polled(Some(()))
                ));
            } else {
                app.dispatch(AppEvent::FeedEvent(
                    FeedEvent::Polled(None)
                ));
            }
        } else {
            if app.feed.is_polling {
                Feed::stop_polling(app, ReasonForStopping::FaultyStop);
            }
            app.dispatch(AppEvent::StateEvent(StateEvent::Error(
                crate::state::ErrorType::CouldNotGetChannel
            )));
        }
    }
}
