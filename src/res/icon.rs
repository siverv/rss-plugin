
use gdk_pixbuf;
use gio;

type Icon = gdk_pixbuf::Pixbuf;


pub fn icon_by_color_and_size(color: &str, size: usize) -> Option<gdk_pixbuf::Pixbuf> {
    let icon_data = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <svg version="1.1"
        width="{}"
        height="{}"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 455.731 455.731">
        <style>
            path, circle {{ fill: {}; }}
        </style>
        <path d="M296.208,159.16C234.445,97.397,152.266,63.382,64.81,63.382v64.348
            c70.268,0,136.288,27.321,185.898,76.931c49.609,49.61,76.931,115.63,76.931,185.898h64.348
            C391.986,303.103,357.971,220.923,296.208,159.16z"/>
        <path d="M64.143,172.273v64.348c84.881,0,153.938,69.056,153.938,153.939h64.348
            C282.429,270.196,184.507,172.273,64.143,172.273z"/>
        <circle cx="109.833" cy="346.26" r="46.088"/>
    </svg>"#, size, size, color);
    let cancellable: Option<&gio::Cancellable> = None;
    let stream = gio::MemoryInputStream::new_from_bytes(&glib::Bytes::from_owned(icon_data.into_bytes()));
    let pixbuf = gdk_pixbuf::Pixbuf::new_from_stream(&stream, cancellable);
    match pixbuf {
        Ok(pixbuf) => Some(pixbuf),
        Err(err) => {
            println!("ERROR: {:?}", err);
            None
        }
    }
}

pub enum IconType { Alert, Idle, Inactive, Notify }
pub struct IconSet {
    pub size: usize,
    icons: (Icon, Icon, Icon, Icon)
}
impl IconSet {

    pub fn new (size: usize) -> Self {
        let colors = ("#900", "#CCC", "rgba(255,255,255,0.2)", "#F60");
        let icons = (
            icon_by_color_and_size(colors.0, size).unwrap(),
            icon_by_color_and_size(colors.1, size).unwrap(),
            icon_by_color_and_size(colors.2, size).unwrap(),
            icon_by_color_and_size(colors.3, size).unwrap()
        );

        IconSet { size, icons }
    }

    pub fn get(&self, icon_type: IconType) -> &Icon {
        match icon_type {
            IconType::Alert => &self.icons.0,
            IconType::Idle => &self.icons.1,
            IconType::Inactive => &self.icons.2,
            IconType::Notify => &self.icons.3,
        }
    }
}
