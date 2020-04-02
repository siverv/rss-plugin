
use crate::app::{App, AppEvent};
use crate::state::{ErrorType, StateEvent};
use glib::SourceId;
use glib::translate::{FromGlib, ToGlib};
use crate::config::Config;

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
    Test,
    Tested(Result<(), ErrorType>),
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
                FeedEvent::Test => Feed::test_config(app),
                FeedEvent::Tested(result) => Feed::handle_test_result(app, result),
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
            app.dispatch(AppEvent::FeedEvent(FeedEvent::Poll));
            return;
        }
        app.feed.is_polling = true;
        let timeout = Feed::get_timeout(app);
        app.dispatch(AppEvent::FeedEvent(FeedEvent::Started));
        app.dispatch(AppEvent::FeedEvent(FeedEvent::Poll));
        {
            let tx = app.tx.clone();
            app.feed.polling_id = Some(gtk::timeout_add(timeout, move || {
                tx.send(AppEvent::FeedEvent(FeedEvent::Poll)).unwrap();
                gtk::Continue(true)
            }));
        }
    }

    fn stop_polling(app: &mut App, reason: ReasonForStopping) {
        if !app.feed.is_polling || !app.config.active {
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

    fn test_config(app: &mut App) {
        if let Some(dialog) = &app.gui.config_dialog {
            let mut tmp_config = app.config.clone();
            if let Err(error) = tmp_config.set_from_dialog(dialog) {
                app.dispatch(AppEvent::FeedEvent(FeedEvent::Tested(Err(error))));
            } else {
                let channel = Feed::read_channel(&tmp_config);
                if channel.is_ok() {
                    app.dispatch(AppEvent::FeedEvent(FeedEvent::Tested(Ok(()))));
                } else if let Err(err) = channel {
                    app.dispatch(AppEvent::FeedEvent(FeedEvent::Tested(Err(err))));
                }
            }
        } else {
            app.dispatch(AppEvent::FeedEvent(FeedEvent::Tested(Err(ErrorType::NoConfigDialog))));
        }
    }

    fn handle_test_result(app: &mut App, result: Result<(), ErrorType>) {
        if let Some(dialog) = &app.gui.config_dialog {
            dialog.update_test_results(result);
        }
    }

    fn read_channel(config: &Config) -> Result<rss::Channel, ErrorType> {
        let feed_url = &config.feed;
        // TODO: Make non-blocking
        let client = reqwest::blocking::Client::new();
        let request = if let crate::config::RequestMethod::Post = config.feed_request_method {
            client.post(feed_url)
        } else {
            client.get(feed_url)
        };
        let request = Feed::apply_headers_to_request(config, request);
        let request = if let crate::config::RequestMethod::Post = config.feed_request_method {
            if let Some(text) = &config.feed_request_body {
                request.body(String::from(text))
            } else {
                request
            }
        } else {
            request
        };
        let content = request.send();
        match content {
            Err(err) => {
                Err(ErrorType::UrlRequestError(err))
            }
            Ok(content) => {
                rss::Channel::read_from(std::io::BufReader::new(content))
                    .map_err(|err| ErrorType::RssError(err))
            }
        }
    }

    fn poll_feed(app: &mut App) {
        if !app.config.active {
            return;
        }
        // TODO: Make polling non-blocking.
        let channel = Feed::read_channel(&app.config);
        if let Ok(channel) = channel {
            let items = channel.items();
            let mut new_items: Vec<rss::Item> = items.iter().filter(|item| {
                !app.feed.all_ids.contains(item.guid().unwrap())
            }).map(|item| item.clone()).collect();

            if app.config.preserve_items {
                new_items.iter().for_each(|item| {
                    app.feed.all_ids.push(item.guid().unwrap().clone());
                });
            } else {
                app.feed.all_ids = items.iter().map(|item| item.guid().unwrap().clone()).collect();
            }
            app.feed.unseen_ids = app.feed.unseen_ids.iter().filter(|id| {
                app.feed.all_ids.contains(id)
            }).chain(new_items.iter().map(|item| item.guid().unwrap())).map(|id| id.clone()).collect();

            if new_items.len() > 0 {
                if app.config.preserve_items {
                    app.feed.items.append(&mut new_items);
                } else {
                    app.feed.items = Vec::from(items);
                }
                app.dispatch(AppEvent::FeedEvent(
                    FeedEvent::Polled(Some(()))
                ));
            } else {
                app.dispatch(AppEvent::FeedEvent(
                    FeedEvent::Polled(None)
                ));
            }
        } else if let Err(err) = channel {
            if app.feed.is_polling {
                Feed::stop_polling(app, ReasonForStopping::FaultyStop);
            }
            app.dispatch(AppEvent::StateEvent(StateEvent::Error(err)));
        }
    }


    fn apply_headers_to_request(config: &Config, mut builder: ::reqwest::blocking::RequestBuilder) -> ::reqwest::blocking::RequestBuilder {
        for (field, value) in config.feed_headers.iter() {
            builder = builder.header(field, value);
        }
        builder
    }
}
