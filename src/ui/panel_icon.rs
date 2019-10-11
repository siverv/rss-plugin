

use gtk::ButtonExt;
use gtk::WidgetExt;


pub fn new_panel_icon() -> (gtk::Button, gtk::Image) {
    let button = gtk::Button::new();
    button.show();

    let image = gtk::ImageBuilder::new()
        .name("toolbar-image")
        .icon_name("rssplugin")
        .build();
    image.show();
    
    button.set_always_show_image(true);
    button.set_image(Some(&image));

    (button, image)
}
