
Xfce4-panel RSS Plugin.
=======================

A simple Xfce4-panel plugin for reading RSS, and notifying on new unread content. The plugin is written with Rust, except for the seven or so lines of C necessary in order to make `XFCE_PANEL_PLUGIN_REGISTER` work. 

Installation:
-------------

```
cargo build --release  # building the rust parts
./make.sh              # creating the plugin librssplugin.so file 
sudo ./install.sh      # moving the .so, .desktop, and .svg files, clear icon cache.
xfce4-panel -r         # restart the panel.
```

The RSS Plugin should now be available to add.


Future:
-------

- Visually pleasing item-view
- Configurable polling-interval
- Configurable item-view, icons and colors
- Better, more ideomatic, rust.
- Remove last dependency of C


Credit:
-------

Using:
- [Rust](https://www.rust-lang.org/)
- [Gtk-rs](https://gtk-rs.org/)
- [Xfce4-panel](https://docs.xfce.org/xfce/xfce4-panel/start)
- [RS's RSS](https://github.com/rust-syndication/rss)
- [Serde.rs](https://serde.rs/). 

Inspired by:
- [The Xfce4 Sample Plugin](https://git.xfce.org/panel-plugins/xfce4-sample-plugin/) and the accompanying [how-to guide for panel plugins](https://wiki.xfce.org/dev/howto/panel_plugins)
- [gDiceRoller](https://gitlab.gnome.org/NoraCodes/gdiceroller/) and the accompanying [tutorial on GTK with rust](https://nora.codes/tutorial/speedy-desktop-apps-with-gtk-and-rust/)


For everything else:
- [Feel free to use, copy, modify, and/or distribute this software for any purpose with or without fee](https://opensource.org/licenses/0BSD)