
/*
* IN PROGRESS.
*/

use libc::{c_void, c_char, c_int};
use glib_sys::{gboolean};
use glib::translate::*;
use glib::{glib_wrapper, glib_object_wrapper};
use crate::xfce::ffi::*;


#[repr(C)]
pub struct _XfcePanelPluginPrivate(c_void);
pub type XfcePanelPluginPrivate = *mut _XfcePanelPluginPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XfcePanelPlugin2 {
    pub bin: gtk_sys::GtkBin,
    pub priv_: *mut XfcePanelPluginPrivate,
}
impl ::std::fmt::Debug for _XfcePanelPlugin2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("_XfcePanelPlugin2 @ {:?}", self as *const _))
            .finish()
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XfcePanelPlugin2Class {
    pub parent_class: gtk_sys::GtkEventBoxClass,

    pub construct: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,

    pub screen_position_changed: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, position: XfceScreenPosition)>,
    pub size_changed: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, size: c_int) -> gboolean>,
    pub orientation_changed: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, orientation: gtk_sys::GtkOrientation)>,
    pub free_data: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,
    pub save: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,
    pub about: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,
    pub configure_plugin: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,
    pub removed: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2)>,
    pub remove_event: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, name: * mut c_char, value: * mut c_void)>,
    pub mode_changed: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, mode: XfcePanelPluginMode) -> gboolean>,
    pub rows_changeds: Option<unsafe extern "C" fn(*mut XfcePanelPlugin2, gtk_sys::GtkOrientation)>,
    pub _gtk_reserved1: Option<unsafe extern "C" fn()>,
    pub _gtk_reserved2: Option<unsafe extern "C" fn()>,
}

//   GtkEventBoxClass __parent__;

//   /*< public >*/
//   /* for object oriented plugins only */
//   void     (*construct)               (XfcePanelPlugin    *plugin);

//   /* signals */
//   void     (*screen_position_changed) (XfcePanelPlugin    *plugin,
//                                        XfceScreenPosition  position);
//   gboolean (*size_changed)            (XfcePanelPlugin    *plugin,
//                                        gint                size);
//   void     (*orientation_changed)     (XfcePanelPlugin    *plugin,
//                                        GtkOrientation      orientation);
//   void     (*free_data)               (XfcePanelPlugin    *plugin);
//   void     (*save)                    (XfcePanelPlugin    *plugin);
//   void     (*about)                   (XfcePanelPlugin    *plugin);
//   void     (*configure_plugin)        (XfcePanelPlugin    *plugin);
//   void     (*removed)                 (XfcePanelPlugin    *plugin);
//   gboolean (*remote_event)            (XfcePanelPlugin    *plugin,
//                                        const gchar        *name,
//                                        const GValue       *value);

//   /* new in 4.10 */
//   void     (*mode_changed)            (XfcePanelPlugin    *plugin,
//                                        XfcePanelPluginMode mode);
//   void     (*nrows_changed)           (XfcePanelPlugin    *plugin,
//                                        guint               rows);

//   /*< private >*/
//   void (*reserved1) (void);
//   void (*reserved2) (void);
// }

impl ::std::fmt::Debug for _XfcePanelPlugin2Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GtkButtonClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .field("construct", &self.construct)
            .field("screen_position_changed", &self.screen_position_changed)
            .field("size_changed", &self.size_changed)
            .field("orientation_changed", &self.orientation_changed)
            .field("free_data", &self.free_data)
            .field("save", &self.save)
            .field("about", &self.about)
            .field("configure_plugin", &self.configure_plugin)
            .field("removed", &self.removed)
            .field("remove_event", &self.remove_event)
            .field("mode_changed", &self.mode_changed)
            .field("rows_changeds", &self.rows_changeds)
            .field("_gtk_reserved1", &self._gtk_reserved1)
            .field("_gtk_reserved2", &self._gtk_reserved2)
            .finish()
    }
}



glib::glib_wrapper! {
    pub struct XfcePanelPlugin2(Object<_XfcePanelPlugin2, _XfcePanelPlugin2Class, XfcePanelPlugin2Class>)
        @extends gtk::EventBox, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || xfce_panel_plugin_get_type(),
    }
}
        // @implements Buildable;



// #[no_mangle]
// pub fn xfce_panel_module_realize(xpp: XfcePanelPluginPointer) {
//     // assert!(gobject_sys::g_check_type_instance_is_a(xpp, xfce_panel_plugin_get_type()));
//     // g_signal_handlers_disconnect_by_func(
//     //     xpp,
//     //     std::boxed::Box::into_raw(xfce_panel_module_realize),
//     //     std::ptr::null()
//     // );
//     constructor(xpp);
// }

// #[no_mangle]
// pub fn xfce_panel_module_construct(xpp_name: * mut c_char,
//                                    xpp_unique_id: c_int,
//                                    xpp_display_name: * mut c_char,
//                                    xpp_comment: * mut c_char,
//                                    xpp_arguments: * mut c_char,
//                                    xpp_screen: gdk_sys::GdkScreen
//                                 ) -> * mut _XfcePanelPlugin2 {
                                
//     // assert!(GDK_IS_SCREEN(xpp_screen));
//     // assert_eq!(xpp_name, std::ptr::null());
//     // assert!(app_unique != -1);
//     // let xpp: glib::glib_wrapper!(
//     //         pub struct XfcePanelPlugin(Shared<ffi::GdkFrameTimings>);

//     //     match fn {
//     //         get_type => || xfce_panel_plugin_get_type(ptr),
//     //     }
//     // );
//     // let xpp: XfcePanelPluginPointer = gobject_sys::g_object_new (xfce_panel_plugin_get_type(),
//     //                         "name".to_glib_none().0, xpp_name,
//     //                         "unique-id".to_glib_none().0, xpp_unique_id,
//     //                         "display-name".to_glib_none().0, xpp_display_name,
//     //                         "comment".to_glib_none().0, xpp_comment,
//     //                         "arguments".to_glib_none().0, xpp_arguments, std::ptr::null());

//     // unsafe extern "C" fn trampoline_onearg<T, F: Fn(T) + 'static>(
//     //     _this: *mut gtk_sys::GtkWidget,
//     //     t: T,
//     //     f: glib_sys::gpointer
//     // ) {
//     //     let f: &F = &*(f as *const F);
//     //     f(t);
//     // }
//     // unsafe {
//     //     connect_raw(
//     //         xpp, 
//     //         "realize".to_glib_none().0,
//     //         Some(transmute(trampoline_onearg::<T, F> as usize)),
//     //         Box_::into_raw(f),
//     //     )
//     // }
//     // XfcePanelPlugin::onearg_connect(xpp, "realize", constructor);
//     unsafe {
//         let name: String = FromGlibPtrFull::from_glib_full(xpp_name);
//         // let unique_id: isize = FromGlib::from_glib(xpp_unique_id);
//         let display_name: String = FromGlibPtrFull::from_glib_full(xpp_display_name);
//         let comment: String = FromGlibPtrFull::from_glib_full(xpp_comment);
//         // let arguments: String = FromGlibPtrFull::from_glib_full(xpp_arguments);
//         // let name: String = FromGlibPtrFull::from_glib_none(xpp_name);
        
//         let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
//         properties.push(("name", &name));
//         properties.push(("unique-id", &xpp_unique_id));
//         properties.push(("display-name", &display_name));
//         properties.push(("comment", &comment));
//         // properties.push(("arguments", &arguments));
//         let plugin: XfcePanelPlugin2 = glib::Object::new(XfcePanelPlugin2::static_type(), &properties)
//             .expect("object new")
//             .downcast()
//             .expect("downcast");
//         println!("HELLO");
//         plugin.as_ptr()
//     }
// }


// #[link(name = "xfce4panel-2.0")]
// #[allow(dead_code)]
// extern {
//     pub fn xfce_panel_plugin_get_type() -> glib_sys::GType;
//     pub fn g_signal_handlers_disconnect_by_func(xpp: XfcePanelPluginPointer, x: *mut gobject_sys::GClosure, y: * mut c_void);
//     pub fn g_signal_connect_after(xpp: XfcePanelPluginPointer, s: *const c_char, y: *mut gobject_sys::GClosure, z: * mut c_void);
// }
