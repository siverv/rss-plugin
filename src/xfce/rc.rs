
use crate::xfce::ffi::*;
use glib::translate::*;
use glib::translate::{ToGlib, FromGlib, FromGlibPtrContainer, FromGlibPtrNone};


#[allow(dead_code)]
pub fn rc_simple<U, F: FnOnce(* const XfceRc) -> U>(filename: &str, f: F) -> U {
    let readonly = true;
    unsafe {
        let rc = xfce_rc_simple_open(filename.to_glib_none().0, readonly.to_glib());
        let res = f(rc.clone());
        xfce_rc_close(rc);
        res
    }
}



#[allow(dead_code)]
pub fn rc_simple_mut<U, F: FnOnce(* mut XfceRc) -> U>(filename: &str, f: F) -> U {
    let readonly = false;
    unsafe {
        let rc = xfce_rc_simple_open(filename.to_glib_none().0, readonly.to_glib());
        let res = f(rc.clone());
        xfce_rc_close(rc);
        res
    }
}

#[allow(dead_code)]
pub fn is_dirty(rc: * const XfceRc) -> bool {
    unsafe {
        FromGlib::from_glib(xfce_rc_is_dirty(rc))
    }
}

#[allow(dead_code)]
pub fn is_readonly(rc: * const XfceRc) -> bool {
    unsafe {
        FromGlib::from_glib(xfce_rc_is_readonly(rc))
    }
}


#[allow(dead_code)]
pub fn get_locale(rc: * const XfceRc) -> String {
    unsafe {
        FromGlibPtrNone::from_glib_none(xfce_rc_get_locale(rc))
    }
}


#[allow(dead_code)]
pub fn get_groups(rc: * const XfceRc) -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(xfce_rc_get_groups(rc)) 
    }
}

#[allow(dead_code)]
pub fn get_entries(rc: * const XfceRc, group: &str) -> Vec<String> {
    unsafe {
       FromGlibPtrContainer::from_glib_none(xfce_rc_get_entries(rc, group.to_glib_none().0))
    }
}


#[allow(dead_code)]
pub fn delete_group(rc: * mut XfceRc, group: &str, global: bool) {
    unsafe {
       xfce_rc_delete_group(rc, group.to_glib_none().0, global.to_glib()) 
    }
}


#[allow(dead_code)]
pub fn get_group(rc: * const XfceRc) -> String {
    unsafe {
        FromGlibPtrNone::from_glib_none(xfce_rc_get_group(rc)) 
    }
}

#[allow(dead_code)]
pub fn has_group(rc: * const XfceRc, group: &str) -> bool {
    unsafe {
        FromGlib::from_glib(xfce_rc_has_group(rc, group.to_glib_none().0))
    }
}

#[allow(dead_code)]
pub fn set_group(rc: * mut XfceRc, group: &str) {
    unsafe {
       xfce_rc_set_group(rc, group.to_glib_none().0)
    }
}


#[allow(dead_code)]
pub fn delete_entry(rc: * mut XfceRc, key: &str, global: bool) {
    unsafe {
       xfce_rc_delete_entry(rc, key.to_glib_none().0, global.to_glib()) 
    }
}

#[allow(dead_code)]
pub fn has_entry(rc: * const XfceRc, key: &str) -> bool {
    unsafe {
        FromGlib::from_glib(xfce_rc_has_entry(rc, key.to_glib_none().0))
    }
}


#[allow(dead_code)]
pub fn read_entry(rc: * const XfceRc, key: &str, fallback: &str) -> String {
    unsafe {
        FromGlibPtrNone::from_glib_none(xfce_rc_read_entry(rc, key.to_glib_none().0, fallback.to_glib_none().0))
    }
}

#[allow(dead_code)]
pub fn read_entry_untranslated (rc: * const XfceRc, key: &str, fallback: &str) -> String {
    unsafe {
        FromGlibPtrNone::from_glib_none(xfce_rc_read_entry_untranslated(rc, key.to_glib_none().0, fallback.to_glib_none().0))
    }
}

#[allow(dead_code)]
pub fn read_bool_entry(rc: * const XfceRc, key: &str, fallback: bool) -> bool {
    unsafe {
        FromGlib::from_glib(xfce_rc_read_bool_entry(rc, key.to_glib_none().0, fallback.to_glib()))
    }
}

#[allow(dead_code)]
pub fn read_int_entry(rc: * const XfceRc, key: &str, fallback: i32) -> i32 {
    unsafe {
        xfce_rc_read_int_entry(rc, key.to_glib_none().0, fallback)
    }
}

#[allow(dead_code)]
pub fn read_list_entry(rc: * const XfceRc, key: &str, delimiter: &str) -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(xfce_rc_read_list_entry(rc, key.to_glib_none().0, delimiter.to_glib_none().0))
    }
}


#[allow(dead_code)]
pub fn write_entry(rc: * mut XfceRc, key: &str, value: &str) {
    unsafe {
       xfce_rc_write_entry(rc, key.to_glib_none().0, value.to_glib_none().0) 
    }
}

#[allow(dead_code)]
pub fn write_bool_entry(rc: * mut XfceRc, key: &str, value: bool) {
    unsafe {
       xfce_rc_write_bool_entry(rc, key.to_glib_none().0, value.to_glib()) 
    }
}

#[allow(dead_code)]
pub fn write_int_entry(rc: * mut XfceRc, key: &str, value: i32) {
    unsafe {
       xfce_rc_write_int_entry(rc, key.to_glib_none().0, value) 
    }
}

#[allow(dead_code)]
pub fn write_list_entry(rc: * mut XfceRc, key: &str, value: Vec<String>, separator: &str) {
    unsafe {
       xfce_rc_write_list_entry(rc, key.to_glib_none().0, value.to_glib_none().0, separator.to_glib_none().0) 
    }
}
