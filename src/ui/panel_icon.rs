

use gtk::ButtonExt;
use gtk::WidgetExt;


#[derive(Shrinkwrap)]
pub struct PanelIcon {
    #[shrinkwrap(main_field)]
    button: gtk::Button,
    pub image: gtk::Image
}

impl PanelIcon {
    pub fn new () -> Self {
        let button = gtk::Button::new();
        button.show();
    
        let image = gtk::ImageBuilder::new()
            .name("toolbar-image")
            .icon_name("rssplugin")
            .build();
        image.show();
        
        button.set_always_show_image(true);
        button.set_image(Some(&image));
        PanelIcon {
            button,
            image
        }
    }

    pub fn as_widget (&self) -> &gtk::Button {
        &self.button
    }
}
