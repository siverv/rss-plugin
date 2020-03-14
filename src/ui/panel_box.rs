

use gtk::ContainerExt;
use gtk::WidgetExt;




#[derive(Shrinkwrap)]
pub struct PanelBox {
    #[shrinkwrap(main_field)]
    event_box: gtk::EventBox,
    pub container: gtk::Box
}

impl PanelBox {
    pub fn new () -> Self {
        let event_box = gtk::EventBox::new();
        event_box.show();
    
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        container.show();
        event_box.add(&container);
    
        PanelBox {
            event_box,
            container
        }
    }

    pub fn as_widget (&self) -> &gtk::EventBox {
        &self.event_box
    }
}
