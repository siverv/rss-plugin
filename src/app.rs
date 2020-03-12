
use crate::state::{State, StateEvent};
use crate::gui::{Gui, GuiEvent};
use crate::config::{Config, ConfigEvent};
use crate::feed::{Feed, FeedEvent};
use crate::xfce::ffi::XfcePanelPluginPointer;


pub enum AppEvent {
    Init,
    StateEvent(StateEvent),
    GuiEvent(GuiEvent),
    ConfigEvent(ConfigEvent),
    FeedEvent(FeedEvent),
}

pub struct App {
    pub tx: glib::Sender<AppEvent>,
    pub state: State,
    pub gui: Gui,
    pub config: Config,
    pub feed: Feed
}

impl App {
    pub fn new (pointer: XfcePanelPluginPointer, tx: glib::Sender<AppEvent>) -> Self {
        let gui = Gui::new(pointer, tx.clone());
        let state = State::new();
        let config = Config::new();
        let feed = Feed::new();
        return App {
            tx,
            state,
            gui,
            config,
            feed
        }
    }

    pub fn reducer(&mut self, event: AppEvent) {
        match event {
            AppEvent::StateEvent(_) => {
                State::reducer(self, event);
            }
            AppEvent::GuiEvent(_) => {
                Gui::reducer(self, event);
            }
            AppEvent::ConfigEvent(_) => {
                Config::reducer(self, event);
            }
            AppEvent::FeedEvent(_) => {
                Feed::reducer(self, event);
            }
            AppEvent::Init => self.init()
        };
        // Handle when to update?
        Gui::update(self);
    }

    fn init(&mut self) {
        Config::init(self);
        Gui::init(self);
        self.dispatch(AppEvent::FeedEvent(FeedEvent::Start));
    }

    pub fn dispatch(&mut self, event: AppEvent) {
        if let Err(_error) = self.tx.send(event) {
            self.state.error = Some(crate::state::ErrorType::CouldNotDispatch);
        }
    }

    pub fn start (pointer: XfcePanelPluginPointer) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let mut app = App::new(pointer, tx);
        app.dispatch(AppEvent::Init);
        rx.attach(None, move |event| {
            app.reducer(event);
            gtk::Continue(true)
        });
    }

}
