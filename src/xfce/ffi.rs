use libc::{c_char, c_int};
use glib_sys::{gboolean};


pub type XfceSize = c_int;
pub type XfcePanelPluginPointer = * mut gtk_sys::GtkWidget;


#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub enum XfceScreenPosition {
    None,
    NorthWestHorizontal,
    North,
    NorthEastHorizontal,
    NorthWestVertical,
    West,
    SouthWestVertical,
    NorthEastVertical,
    East,
    SouthEastVertical,
    SouthWestHorizontal,
    South,
    SouthEastHorizontal,
    FloatingHorizontal,
    FloatingVertical,
}


#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub enum XfcePanelPluginMode {
    Horizontal,
    Vertical,
    Deskbar
}

#[link(name = "xfce4panel-2.0")]
#[allow(dead_code)]
extern {
    pub fn xfce_panel_plugin_get_type() -> glib_sys::GType;

    pub fn xfce_panel_plugin_get_name(plugin: XfcePanelPluginPointer) -> * mut c_char;
    pub fn xfce_panel_plugin_get_id(plugin: XfcePanelPluginPointer) -> * mut c_char;
    pub fn xfce_panel_plugin_get_display_name(plugin: XfcePanelPluginPointer) -> * mut c_char;
    
    pub fn xfce_panel_plugin_get_orientation(plugin: XfcePanelPluginPointer) -> gtk_sys::GtkOrientation;
    pub fn xfce_panel_plugin_get_screen_position(plugin: XfcePanelPluginPointer) -> XfceScreenPosition;
    pub fn xfce_panel_plugin_get_size(plugin: XfcePanelPluginPointer) -> c_int;
    pub fn xfce_panel_plugin_get_expand(plugin: XfcePanelPluginPointer) -> gboolean;
    pub fn xfce_panel_plugin_set_expand(plugin: XfcePanelPluginPointer, expand: gboolean);

    pub fn xfce_panel_plugin_menu_show_about(plugin: XfcePanelPluginPointer); 
    pub fn xfce_panel_plugin_menu_show_configure(plugin: XfcePanelPluginPointer);

    pub fn xfce_panel_plugin_add_action_widget (plugin: XfcePanelPluginPointer, widget: * mut gtk_sys::GtkWidget); 
    pub fn xfce_panel_plugin_menu_insert_item (plugin: XfcePanelPluginPointer, item: * mut gtk_sys::GtkMenuItem); 


    pub fn xfce_panel_plugin_lookup_rc_file (plugin: XfcePanelPluginPointer) -> * mut c_char;
    pub fn xfce_panel_plugin_save_location (plugin: XfcePanelPluginPointer, create: gboolean) -> * mut c_char;

    
}

pub type XfceRc = gobject_sys::GObject;

#[link(name = "xfce4util")]
#[allow(dead_code)]
extern {
    pub fn xfce_rc_get_type() -> glib_sys::GType;
    
    pub fn xfce_rc_simple_open(filename: * const c_char, readonly: gboolean) -> * mut XfceRc;
    pub fn xfce_rc_config_open(filename: * const c_char, resource: * const c_char, readonly: gboolean) -> * mut XfceRc;

    pub fn xfce_rc_close (rc: * mut XfceRc);
    pub fn xfce_rc_flush (rc: * mut XfceRc);
    pub fn xfce_rc_rollback (rc: * mut XfceRc);

    pub fn xfce_rc_is_dirty(rc: * const XfceRc) -> gboolean;
    pub fn xfce_rc_is_readonly(rc: * const XfceRc) -> gboolean;

    pub fn xfce_rc_get_locale(rc: * const XfceRc) -> * const c_char;

    pub fn xfce_rc_get_groups(rc: * const XfceRc) -> * const * const c_char;
    pub fn xfce_rc_get_entries(rc: * const XfceRc, group: * const c_char) -> * const * const c_char;

    pub fn xfce_rc_delete_group(rc: * mut XfceRc, group: * const c_char, global: gboolean);

    pub fn xfce_rc_get_group(rc: * const XfceRc) -> * const c_char;
    pub fn xfce_rc_has_group(rc: * const XfceRc, group: * const c_char) -> gboolean;
    pub fn xfce_rc_set_group(rc: * mut XfceRc, group: * const c_char);

    pub fn xfce_rc_delete_entry(rc: * mut XfceRc, key: * const c_char, global: gboolean);
    pub fn xfce_rc_has_entry(rc: * const XfceRc, key: * const c_char) -> gboolean;

    pub fn xfce_rc_read_entry(rc: * const XfceRc, key: * const c_char, fallback: * const c_char) -> * const c_char;
    pub fn xfce_rc_read_entry_untranslated (rc: * const XfceRc, key: * const c_char, fallback: * const c_char) -> * const c_char;
    pub fn xfce_rc_read_bool_entry(rc: * const XfceRc, key: * const c_char, fallback: gboolean) -> gboolean;
    pub fn xfce_rc_read_int_entry(rc: * const XfceRc, key: * const c_char, fallback: c_int) -> c_int;
    pub fn xfce_rc_read_list_entry(rc: * const XfceRc, key: * const c_char, delimiter: * const c_char) -> * const * const c_char;

    pub fn xfce_rc_write_entry(rc: * mut XfceRc, key: * const c_char, value: * const c_char);
    pub fn xfce_rc_write_bool_entry(rc: * mut XfceRc, key: * const c_char, value: gboolean);
    pub fn xfce_rc_write_int_entry(rc: * mut XfceRc, key: * const c_char, value: c_int);
    pub fn xfce_rc_write_list_entry(rc: * mut XfceRc, key: * const c_char, value: * const * const c_char, separator: * const c_char);
}
