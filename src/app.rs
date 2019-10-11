
use std::sync::Arc;
use std::cell::{RefCell};
use crate::gui::Gui;
use crate::state::State;
use crate::config::Config;
use crate::xfce::ffi::XfcePanelPluginPointer;

use gtk::ButtonExt;
use gtk::WidgetExt;
use std::convert::TryInto;



pub struct App {
    pub gui: Gui,
    pub state: State,
    pub config: Config,
}

impl App {
    #[inline]
    pub fn update<U, F: FnOnce(&mut Gui, &mut State, &mut Config) -> U>(&mut self, f: F ) {
        {
            f(&mut self.gui, &mut self.state, &mut self.config);
        }
        Gui::update(&self.gui, &self.state, &self.config);
    }

    pub fn with_mut<U, F: FnOnce(&mut Gui, &mut State, &mut Config) -> U>(&mut self, f: F) -> U {
        f(&mut self.gui, &mut self.state, &mut self.config)
    }
    pub fn with<U, F: FnOnce(&Gui, &State, &Config) -> U>(&self, f: F) -> U {
        f(&self.gui, &self.state, &self.config)
    }

    pub fn start (pointer: XfcePanelPluginPointer) {
        let gui = Gui::new(pointer);
        gui.plugin.save_location(true);
        gui.plugin.save_location(true);
        gui.plugin.save_location(true);
        gui.plugin.save_location(true);
        let rc_file = gui.plugin.save_location(true);
        let mut state = State::new();
        let config = match Config::new(&rc_file) {
            Ok(config) => config,
            Err(err) => {
                if let Some(file) = rc_file.clone() {
                    println!("Error opening config file at {}: {}", file, err);
                } else {
                    println!("Error opening config file: {}", err);
                }
                state.error = Some(crate::state::ErrorType::CouldNotReadConfigFile);
                Config::default()
            }
        };

        let app = App { gui, state, config };

        let app_ref = Arc::new(RefCell::new(app));

        {
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.plugin.connect_orientation_changed(move |orientation| {
                app_ref.borrow_mut().update(|_, state, _| {
                    state.set_orientation(orientation);
                });
            });
        }
        {
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.plugin.connect_screen_position_changed(move |screen_position| {
                app_ref.borrow_mut().update(|_, mut state, _| {
                    state.position = screen_position;
                });
            });
        }
        {
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.plugin.connect_size_changed(move |size| {
                app_ref.borrow_mut().update(|gui, mut state, _| {
                    state.size = size;
                    gui.recreate_icons_in_size(size.try_into().unwrap());
                });
            });
        }
        {   
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.plugin.connect_about(move || {
                app_ref.borrow().with(Gui::open_about);
            });
        }
        {
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.plugin.connect_configure(move || {
                let config_dialog = app_ref.borrow().with(Gui::open_configure);
                crate::config::Config::connect_save_if_confirmed(app_ref.clone(), config_dialog);
            });
        }
    
        {
            let app_ref = app_ref.clone();
            app_ref.clone().borrow().gui.button.connect_clicked(move |_button| {
                app_ref.borrow_mut().with_mut(|_, mut state, _| state.seen_ids = state.ids.clone());
                {
                    let mut existing_dialog;
                    {
                        existing_dialog = app_ref.borrow().gui.item_list_popup.clone();
                    }
                    if let Some(dialog) = existing_dialog {
                        dialog.destroy();
                        app_ref.borrow_mut().with_mut(|mut gui, _, _| gui.item_list_popup = None);
                    } else {
                        let feed_dialog = app_ref.borrow().with(Gui::open_feed);
                        {
                            app_ref.borrow_mut().with_mut(|mut gui, _, _| gui.item_list_popup = Some(feed_dialog.clone()));
                        }
                        {
                            let app_ref = app_ref.clone();
                            feed_dialog.connect_destroy(move |_dialog| {
                                app_ref.borrow_mut().with_mut(|mut gui, _, _| gui.item_list_popup = None);
                            });
                        };
                    }
                }
                // {
                //     if !app_ref.borrow().state.is_polling {
                //         crate::feed::start_polling_feed(app_ref.clone());
                //     };
                // } 
                app_ref.borrow_mut().update(|_,_,_|{});
            });
        }


        crate::feed::start_polling_feed(app_ref.clone());

        app_ref.borrow().gui.start();
    }
}
