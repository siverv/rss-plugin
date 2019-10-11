

use gtk;
use gtk::DialogExt;
use gtk::WidgetExt;


pub fn open_about_dialog(parent: &gtk::Window) -> gtk::AboutDialog {
    let dialog = gtk::AboutDialogBuilder::new()
        .title("About")
        .version(crate::res::VERSION)
        .program_name(crate::res::APP_NAME)
        .window_position(gtk::WindowPosition::Center)
        .transient_for(parent)
        .destroy_with_parent(true)
        .logo_icon_name("rssplugin")
        .icon_name("xfce4-about")
        .build();
    dialog.connect_response(|dialog, _| dialog.destroy());
    dialog.show();
    dialog
}
