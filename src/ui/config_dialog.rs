

use gtk::GtkWindowExt;
use gtk::DialogExt;
use gtk::EntryExt;
use gtk::ToggleButtonExt;
use gtk::WidgetExt;
use gtk::LabelExt;
use gtk::ContainerExt;
use gtk::ButtonExt;
use gtk::TextBufferExt;
use gtk::TextViewExt;
use gtk::NotebookExt;
use crate::app::App;

use gtk::prelude::*;
use gtk::{ListStore, TreeViewColumn};


#[derive(Shrinkwrap)]
pub struct ConfigDialog {
    #[shrinkwrap(main_field)]
    dialog: gtk::Dialog,
    location_label: gtk::Label,
    active: gtk::CheckButton,
    preserve_items: gtk::CheckButton,
    feed_tab: FeedTab,
    polling_interval: PollingInterval

}

impl ConfigDialog {
    pub fn new (parent: &gtk::Window) -> Self {
        let dialog = gtk::Dialog::new_with_buttons(
            Some("Rss Plugin Configuration"),
            Some(parent),
            gtk::DialogFlags::DESTROY_WITH_PARENT,
            &[
                ("gtk-close", gtk::ResponseType::Close),
                ("gtk-save", gtk::ResponseType::Accept)
            ]
        );
        dialog.set_position(gtk::WindowPosition::Center);
        dialog.set_icon_name(Some("xfce4-settings"));

        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
        container.show();
        let notebook = gtk::Notebook::new();
        notebook.show();
        notebook.set_size_request(500,300);
        notebook.add(&container);
        
        let feed_tab = FeedTab::new();
        notebook.add(feed_tab.as_widget());
        notebook.set_tab_label(feed_tab.as_widget(), Some(&feed_tab.title()));

        let notebook_tab_label = gtk::Label::new(Some("General"));
        notebook_tab_label.show();
        notebook.set_tab_label(&container, Some(&notebook_tab_label));
    
        dialog.get_content_area().add(&notebook);
    
        dialog.connect_response(|dialog, response| {
            match response {
                gtk::ResponseType::Accept => {},
                _ => {dialog.destroy();}
            }
        });

        let location_label = gtk::Label::new(Some("No save location found. Config might not be preserved"));
        location_label.show();

        let active = gtk::CheckButtonBuilder::new()
            .label("Active")
            .name("active")
            .build();
        active.show();
        let preserve_items = gtk::CheckButtonBuilder::new()
            .label("Preserve items")
            .name("preserve-items")
            .build();
        preserve_items.show();

        let polling_interval = PollingInterval::new();


        container.add(&location_label);
        container.add(&active);
        container.add(&preserve_items);
        container.add(polling_interval.as_widget());

        

        ConfigDialog {
            dialog,
            location_label,
            feed_tab,
            active,
            preserve_items,
            polling_interval
        }
    }

    pub fn update(&self, app: &App) {
        if let Some(loc) = &app.config.save_location {
            self.location_label.set_label(&format!("Save location: {}",loc));
        }
        self.feed_tab.uri_entry.set_text(&app.config.feed);
        self.active.set_active(app.config.active);
        self.polling_interval.set(app.config.polling_interval);
        self.feed_tab.headers_editor.set_headers(&app.config.feed_headers);
        self.feed_tab.request_method.set_request_method(&app.config.feed_request_method);
        self.feed_tab.request_body.set_request_body(&app.config.feed_request_method, &app.config.feed_request_body);
    }

    pub fn get_active(&self) -> bool {
        self.active.get_active()
    }

    pub fn get_preserve_items(&self) -> bool {
        self.preserve_items.get_active()
    }

    pub fn get_polling_interval(&self) -> Result<u32, ()> {
        self.polling_interval.get()
    }

    pub fn get_feed(&self) -> String {
        if let Some(value) = &self.feed_tab.uri_entry.get_text() {
            String::from(value.as_str())
        } else {
            String::new()
        }
    }

    pub fn connect_test_feed<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.feed_tab.connect_test(f);
    }

    pub fn update_test_results(&self, result: Result<(), crate::state::ErrorType>) {
        self.feed_tab.update_test_results(result);
    }
    
    pub fn get_headers(&self) -> Vec<(String, String)> {
        self.feed_tab.headers_editor.get_headers()
    }

    pub fn get_request_method(&self) -> crate::config::RequestMethod {
        self.feed_tab.request_method.get_request_method()
    }

    pub fn get_request_body(&self, method: &crate::config::RequestMethod) -> Option<String> {
        self.feed_tab.request_body.get_request_body(method)
    }
}




#[derive(Shrinkwrap)]
pub struct FeedTab {
    #[shrinkwrap(main_field)]
    container: gtk::Box,
    pub uri_entry: FeedURIEntry,
    pub headers_editor: FeedHeadersList,
    pub request_method: FeedRequestMethod,
    pub request_body: FeedRequestBody,
    test_button: gtk::Button,
    test_result: gtk::TextView
}

impl FeedTab {
    pub fn title(&self) -> gtk::Label {
        let title = gtk::Label::new(Some("Feed source"));
        title.show();
        title
    }
    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
        container.show();
        let uri_entry = FeedURIEntry::new();
        container.add(uri_entry.as_widget());

        // let advanced = gtk::Expander::new(Some("Advanced feed configuration"));
        // advanced.show();
        // let advanced_inner = gtk::Box::new(gtk::Orientation::Vertical, 5);
        // advanced_inner.show();
        // advanced.add(&advanced_inner);

        let request_method = FeedRequestMethod::new();
        container.add(request_method.as_widget());

        let request_body = FeedRequestBody::new();
        container.add(request_body.as_widget());
        request_method.toggle_body_with_activation(request_body.as_widget().clone());

        let headers_editor = FeedHeadersList::new();
        container.add(headers_editor.as_widget());
        
        let test_button = gtk::ButtonBuilder::new()
        .label("Test")
        .build();
        test_button.show();
        let test_result = gtk::TextView::new();
        let header_buffer = test_result.get_buffer();
        if let Some(buffer) = header_buffer {
            buffer.set_text("");
        }
        container.add(&test_button);
        container.add(&test_result);
        
        // container.add(&advanced);
        FeedTab {
            container,
            uri_entry,
            test_button,
            test_result,
            request_method,
            request_body,
            headers_editor
        }
    }


    pub fn connect_test<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.test_button.connect_clicked(move |button| {
            button.set_label("Testing...");
            f(button)
        });
    }
    pub fn update_test_results(&self, result: Result<(), crate::state::ErrorType>) {
        if let Err(err) = result {
            self.test_button.set_label("Test: Err");
            self.test_button.set_tooltip_text(Some(&format!("{}", err)));
            self.test_result.show();
            if let Some(buffer) = self.test_result.get_buffer() {
                buffer.set_text(&format!("{}", err));
            }

        } else {
            self.test_button.set_label("Test: Ok");
            self.test_result.hide();
        }
    }
}








#[derive(Shrinkwrap)]
pub struct FeedURIEntry {
    #[shrinkwrap(main_field)]
    entry: gtk::Entry,
    container: gtk::Box
}

impl FeedURIEntry {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.show();

        let label = gtk::LabelBuilder::new()
            .label("Feed URI:")
            .build();
        label.show();
        container.add(&label);

        let entry = gtk::EntryBuilder::new()
            .placeholder_text("https://...")
            .build();
        entry.show();
        entry.set_hexpand(true);
        container.add(&entry);
        
        FeedURIEntry {
            container,
            entry
        }
    }


    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }

}



#[derive(Shrinkwrap)]
pub struct FeedHeadersList {
    #[shrinkwrap(main_field)]
    list_view: gtk::TreeView,
    list_store: gtk::ListStore,
    container: gtk::Box
}

impl FeedHeadersList {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let scrolling_area = gtk::ScrolledWindowBuilder::new()
            .vexpand(true)
            .hexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        scrolling_area.show();
        scrolling_area.set_size_request(100, 100);
    
        let scrolling_area_inner = gtk::Box::new(gtk::Orientation::Vertical, 10);
        scrolling_area.add(&scrolling_area_inner);
        scrolling_area_inner.show();
        
        let list_view = gtk::TreeView::new();
        let list_store = gtk::ListStore::new(&[gtk::Type::String, gtk::Type::String]);

        // list_store.insert_with_values(None, &[0, 1], &[&"X-Custom-Header", &"Hello World"]);

        list_view.set_model(Some(&list_store));
    
        let field_name_column = FeedHeadersList::create_column(&list_store, 0, "Header Field Name");
        list_view.append_column(&field_name_column);
    
        let field_value_column = FeedHeadersList::create_column(&list_store, 1, "Header Field Value");
        list_view.append_column(&field_value_column);

        // let field_value_column = FeedHeadersList::create_button_column(&list_store, 2, "Remove", cross);
        // list_view.append_column(&field_value_column);

        // {
        //     let list_store = list_store.clone(); 
        //     list_view.connect_row_activated(move |_view, path, _column|{
        //         let indices = path.get_indices();
        //         if let Some(iter) = list_store.get_iter(&path) {
        //             list_store.set_value(&iter, 0, &gtk::Value::from(&format!("{} {:?}", 1, indices)));
        //             let row = list_store.get_value(&iter, 2).get::<u32>();
        //             if let Some(row) = row {
        //                 list_store.set_value(&iter, 0, &gtk::Value::from(&"YWLO"));
        //             }
        //         }
        //     });
        // }

        scrolling_area_inner.add(&list_view);

        let button_row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let add_row_button = gtk::Button::new();
        add_row_button.set_label("Add header row");
        button_row.add(&add_row_button);
        {
            let list_store = list_store.clone(); 
            add_row_button.connect_clicked(move |_|{
                list_store.insert_with_values(None, &[0, 1], &[&"", &""]);
            });
        }
        let del_row_button = gtk::Button::new();
        del_row_button.set_label("Remove header row");
        {
            let list_store = list_store.clone(); 
            del_row_button.connect_clicked(move |_|{
                let last_index = list_store.iter_n_children(None);
                if last_index > 0 {
                    if let Some(iter) = list_store.iter_nth_child(None, last_index-1) {
                        list_store.remove(&iter);
                    }
                }
            });
        }
        button_row.add(&del_row_button);


        let label = gtk::LabelBuilder::new()
        .label("<b>HTTP Request Headers</b>")
        .use_markup(true)
        .build();
        label.show();
        container.add(&label);

        container.add(&button_row);
        container.add(&scrolling_area);

        list_view.expand_all();
        container.show_all();

        FeedHeadersList {
            list_view,
            list_store,
            container
        }
    }

    fn create_column(list_store: &gtk::ListStore, col: u32, title: &str) -> gtk::TreeViewColumn {
        let object_to_render_cells = gtk::CellRendererText::new();
        object_to_render_cells.set_visible(true);
        object_to_render_cells.set_property_editable(true);
        {
            let list_store = list_store.clone(); 
            object_to_render_cells.connect_edited(move |_, path, value|{
                if let Some(iter) = list_store.get_iter(&path) {
                    list_store.set_value(&iter, col, &gtk::Value::from(value));
                }
            });
        }
        let view_column= TreeViewColumn::new();
        view_column.set_expand(true);
        view_column.set_visible(true);
        view_column.set_title(title);
        view_column.pack_start(&object_to_render_cells,true);
        view_column.add_attribute(&object_to_render_cells,"text",col as i32);
        view_column
    }

    // fn create_button_column(list_store: &gtk::ListStore, col: u32, title: &str, icon: Option<gdk_pixbuf::Pixbuf>) -> gtk::TreeViewColumn {
    //     let object_to_render_cells = gtk::CellRendererPixbuf::new();
    //     object_to_render_cells.set_visible(true);
    //     let view_column= TreeViewColumn::new();
    //     view_column.set_visible(true);
    //     view_column.set_clickable(true);
    //     view_column.set_title(title);
    //     view_column.pack_start(&object_to_render_cells,true);
    //     view_column.add_attribute(&object_to_render_cells,"pixbuf",col as i32);
    //     view_column
    // }

    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }
    
    pub fn set_headers(&self, headers: &Vec<(String, String)>) {
        let n = self.list_store.iter_n_children(None);
        if n > 0 {
            for i in 0..n {
                if let Some(iter) = self.list_store.iter_nth_child(None, i) {
                    self.list_store.remove(&iter);
                }
            }
        }
        for (name, value) in headers {
            self.list_store.insert_with_values(None, &[0, 1], &[name, value]);
        }
    }

    pub fn get_headers(&self) -> Vec<(String, String)> {
        let mut headers: Vec<(String, String)> = Vec::new();
        let rows = self.list_store.iter_n_children(None);
        for row in 0..rows {
            if let Some(iter) = self.list_store.iter_nth_child(None, row) {
                let field: String = self.list_store.get_value(&iter, 0).get().unwrap();
                let value: String = self.list_store.get_value(&iter, 1).get().unwrap();
                headers.push((field, value));
            }
        }
        headers
    }
}



#[derive(Shrinkwrap)]
pub struct FeedRequestMethod {
    #[shrinkwrap(main_field)]
    combo_box: gtk::ComboBoxText,
    container: gtk::Box,
}

impl FeedRequestMethod {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.show();

        let label = gtk::LabelBuilder::new()
            .label("Request method:")
            .build();
        label.show();
        container.add(&label);

        let list_store = ListStore::new(&[gtk::Type::String]);
        list_store.insert_with_values(None, &[0], &[&"GET"]);
        list_store.insert_with_values(None, &[0], &[&"POST"]);

        let combo_box = gtk::ComboBoxText::new();
        FeedRequestMethod::add_method_to_combo_box(&combo_box, crate::config::RequestMethod::Get);
        FeedRequestMethod::add_method_to_combo_box(&combo_box, crate::config::RequestMethod::Post);
        combo_box.set_active_id(Some(&format!("{}", crate::config::RequestMethod::Get)));
        combo_box.show();
        combo_box.set_hexpand(true);
        container.add(&combo_box);
        
        FeedRequestMethod {
            combo_box,
            container
        }
    }

    fn add_method_to_combo_box(combo_box: &gtk::ComboBoxText, method: crate::config::RequestMethod) {
        let value = format!("{}", method);
        combo_box.append(Some(&value), &value);
    }


    pub fn as_widget(&self) -> &gtk::Box {
        &self.container

    }

    pub fn set_request_method(&self, method: &crate::config::RequestMethod) {
        self.combo_box.set_active_id(Some(&format!("{}", method)));
    }

    pub fn get_request_method(&self) -> crate::config::RequestMethod {
        if let Some(value) = self.combo_box.get_active_id() {
            crate::config::RequestMethod::from(&value.to_string()[..])
        } else {
            crate::config::RequestMethod::Get
        }
    }

    pub fn toggle_body_with_activation(&self, body_container: gtk::Box) {
        self.combo_box.connect_changed(move |combo_box| {
            if let Some(value) = combo_box.get_active_id() {
                let method = crate::config::RequestMethod::from(&value.to_string()[..]);
                match method {
                    crate::config::RequestMethod::Get => {
                        body_container.hide();
                    }
                    crate::config::RequestMethod::Post => {
                        body_container.show();
                    }
                }
            }
        });
    }
}


#[derive(Shrinkwrap)]
pub struct FeedRequestBody {
    #[shrinkwrap(main_field)]
    request_body: gtk::TextView,
    container: gtk::Box
}

impl FeedRequestBody {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.show();
        

        let label = gtk::LabelBuilder::new()
            .label("Request body:")
            .build();
        label.show();
        container.add(&label);


        let request_body = gtk::TextView::new();
        let header_buffer = request_body.get_buffer();
        if let Some(buffer) = header_buffer {
            buffer.set_text("");
        }
        request_body.set_hexpand(true);
        request_body.show();
        container.add(&request_body);
        
        FeedRequestBody {
            request_body,
            container,
        }
    }


    pub fn as_widget(&self) -> &gtk::Box {
        &self.container

    }

    pub fn set_request_body(&self, method: &crate::config::RequestMethod, body: &Option<String>) {
        match method {
            crate::config::RequestMethod::Get => {
                self.container.hide();
            }
            crate::config::RequestMethod::Post => {
                self.container.show();
                self.set_body(body);
            }
        }
    }

    pub fn get_request_body(&self, method: &crate::config::RequestMethod) -> Option<String> {
        match method {
            crate::config::RequestMethod::Get => {
                None
            }
            crate::config::RequestMethod::Post => {
                self.get_body()
            }
        }
    }

    fn set_body(&self, body: &Option<String>) {
        let header_buffer = self.request_body.get_buffer();
        if let Some(buffer) = header_buffer {
            if let Some(text) = body {
                buffer.set_text(text);
            }
        }
    }

    fn get_body(&self) -> Option<String> {
        let header_buffer = self.request_body.get_buffer();
        if let Some(buffer) = header_buffer {
            let (start, end) = buffer.get_bounds();
            if let Some(text) = buffer.get_text(&start, &end, false) {
                Some(text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}




#[derive(Shrinkwrap)]
pub struct PollingInterval {
    #[shrinkwrap(main_field)]
    entry: gtk::Entry,
    container: gtk::Box
}

impl PollingInterval {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
        container.show();
        let inner_container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        inner_container.show();

        let label = gtk::LabelBuilder::new()
            .label("Polling Interval:")
            .build();
        label.show();
        inner_container.add(&label);

        let entry = gtk::EntryBuilder::new()
            .placeholder_text("https://...")
            .build();
        entry.show();
        entry.set_hexpand(true);
        inner_container.add(&entry);


        let label = gtk::LabelBuilder::new()
            .label("(eg. 1h 30m 2s)")
            .build();
        label.show();
        inner_container.add(&label);

        container.add(&inner_container);

        let info_message = gtk::LabelBuilder::new()
            .label("Valid units: s=second, m=minute, h=hour, d=day. Minimum 30s")
            .build();
        info_message.show();
        entry.set_hexpand(true);
        container.add(&info_message);

        // let error_message = gtk::LabelBuilder::new()
        //     .label("")
        //     .build();
        // error_message.override_color(gtk::StateFlags::NORMAL, Some(&gdk::RGBA::red()));
        // entry.set_hexpand(true);
        // container.add(&error_message);
        
        PollingInterval {
            container,
            entry,
        }
    }


    pub fn as_widget(&self) -> &gtk::Box {
        &self.container
    }
    
    const SECOND: u32 = 1000;
    const MINUTE: u32 = 60 * Self::SECOND;
    const HOUR: u32 = 60 * Self::MINUTE;
    const DAY: u32 = 24 * Self::HOUR;
    fn join_value(value: u32) -> Result<String, ()> {
        let days = value / Self::DAY;
        let value = value - days * Self::DAY;
        let hours = value / Self::HOUR;
        let value = value - hours * Self::HOUR;
        let minutes = value / Self::MINUTE;
        let value = value - minutes * Self::MINUTE;
        let seconds = value / Self::SECOND;
        let mut units: Vec<String> = Vec::new();
        if days > 0 {
            units.push(format!("{}d", days));
        }
        if hours > 0 {
            units.push(format!("{}h", hours));
        }
        if minutes > 0 {
            units.push(format!("{}m", minutes));
        }
        if seconds > 0 {
            units.push(format!("{}s", seconds));
        }
        Ok(units.join(" "))
    }
    fn parse_value(text: &str) -> Result<u32,()> {
        // TODO: Error handling
        let value = text.split_whitespace().map(|text_unit| {
            if text_unit.len() < 2 {
                0
            } else {
                let (value, unit) = text_unit.split_at(text_unit.len() - 1);
                value.parse::<u32>().map(|value| {
                    match unit {
                        "d" => Ok(value * Self::DAY),
                        "h" => Ok(value * Self::HOUR),
                        "m" => Ok(value * Self::MINUTE),
                        "s" => Ok(value * Self::SECOND),
                        _ => Err(())
                    }
                }).unwrap_or(Ok(0)).unwrap_or(0)
            }
        }).fold(0, |x,y| x + y);
        Ok(value)
    }

    pub fn set(&self, value: u32) {
        // TODO: Do better
        let value = if value < 30 * Self::SECOND {
            30 * Self::SECOND
        } else {value};
        if let Ok(text) = Self::join_value(value) {
            self.entry.set_text(&text)
        }
    }

    pub fn get(&self) -> Result<u32, ()> {
        if let Some(text) = self.entry.get_text() {
            Self::parse_value(&text.to_string()[..]).map(|value |{
                if value < 30 * Self::SECOND {
                    30 * Self::SECOND
                } else {
                    value
                }
            })
        } else {
            Err(())
        }
    }
}
