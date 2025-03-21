#![deny(unused_imports)]

use objc2::{class, msg_send};
use objc2_app_kit::NSRunningApplication;
use objc2_core_foundation::{
    CFArray, CFArrayGetCount, CFArrayGetValueAtIndex, CFBoolean, CFBooleanGetValue, CFDictionary,
    CFDictionaryContainsKey, CFDictionaryGetValue, CFNumber, CFNumberGetValue, CFNumberType,
    CFRetained, CFString, CGRect,
};
use objc2_core_graphics::{
    CGRectMakeWithDictionaryRepresentation, CGWindowListCopyWindowInfo, CGWindowListOption,
};
use std::ffi::c_void;

use crate::winx::core::{ActiveWindow, Api, WindowPosition};

pub struct MacosAPI {}

impl Api for MacosAPI {
    fn get_active_window(&self) -> ActiveWindow {
        let windows: Vec<ActiveWindow> = get_windows_informations(true);

        if !windows.is_empty() {
            let t: &ActiveWindow = windows.first().unwrap();
            t.clone() as ActiveWindow
        } else {
            ActiveWindow::empty()
        }
    }

    fn get_windows(&self) -> Vec<ActiveWindow> {
        get_windows_informations(false)
    }

    fn activate(&self, window_id: String) {
        let process_id = window_id.parse::<i32>().unwrap();
       
        let app: &NSRunningApplication = unsafe {
            msg_send![
              class!(NSRunningApplication),
              runningApplicationWithProcessIdentifier: process_id
            ]
        };

        // unsafe { msg_send![app, activateWithOptions: 1] };

    }
}

fn get_windows_informations(only_active: bool) -> Vec<ActiveWindow> {
    let mut windows: Vec<ActiveWindow> = Vec::new();

    let options = CGWindowListOption::OptionOnScreenOnly
        | CGWindowListOption::ExcludeDesktopElements
        | CGWindowListOption::OptionIncludingWindow;

    let window_list_info: &CFArray = unsafe { &CGWindowListCopyWindowInfo(options, 0).unwrap() };

    let windows_count = unsafe { CFArrayGetCount(window_list_info) };

    // let screen_rect = get_screen_rect();

    for idx in 0..windows_count {
        let window_cf_dictionary_ref =
            unsafe { CFArrayGetValueAtIndex(window_list_info, idx) as *const CFDictionary };

        if window_cf_dictionary_ref.is_null() {
            continue;
        }
        let window_cf_dictionary =
            unsafe { CFRetained::retain(std::ptr::NonNull::from(&*window_cf_dictionary_ref)) };
        let is_screen: bool = get_cf_boolean_value(&window_cf_dictionary, "kCGWindowIsOnscreen");
        if !is_screen {
            continue;
        }

        let window_layer = get_cf_number_value(&window_cf_dictionary, "kCGWindowLayer");

        if window_layer.lt(&0) || window_layer.gt(&100) {
            continue;
        }

        let bounds = get_cf_window_bounds_value(&window_cf_dictionary);

        if bounds.is_none() {
            continue;
        }

        let bounds = bounds.unwrap();

        if bounds.size.height.lt(&50.0) || bounds.size.width.lt(&50.0) {
            continue;
        }

        let process_id = get_cf_number_value(&window_cf_dictionary, "kCGWindowOwnerPID");
        if process_id == 0 {
            continue;
        }

        let app: &NSRunningApplication = unsafe {
            msg_send![
              class!(NSRunningApplication),
              runningApplicationWithProcessIdentifier: process_id
            ]
        };

        let is_not_active = !unsafe { app.isActive() };

        if only_active && is_not_active {
            continue;
        }

        let bundle_identifier = get_bundle_identifier(app);

        if bundle_identifier.eq("com.apple.dock") {
            continue;
        }

        let app_name = get_cf_string_value(&window_cf_dictionary, "kCGWindowOwnerName");
        let title = get_cf_string_value(&window_cf_dictionary, "kCGWindowName");

        let path: String = unsafe {
            match app.bundleURL() {
                Some(nsurl) => match nsurl.path() {
                    Some(path) => path.to_string(),
                    None => String::from(""),
                },
                None => String::from(""),
            }
        };

        let exec_name: String = {
            match path.is_empty() {
                true => match std::path::Path::new(&path).file_name() {
                    Some(os_str) => match os_str.to_str() {
                        Some(exec_name) => exec_name.to_owned(),
                        None => String::from(""),
                    },
                    None => String::from(""),
                },
                false => String::from(""),
            }
        };

        let memory = get_cf_number_value(&window_cf_dictionary, "kCGWindowMemoryUsage");
        let window_id = get_cf_number_value(&window_cf_dictionary, "kCGWindowNumber");

        windows.push(ActiveWindow {
            app_name: app_name.to_owned(),
            title,
            exec_name,
            window_id: window_id.to_string(),
            process_id: process_id as u32,
            memory: memory as u32,
            position: WindowPosition::new(
                bounds.origin.x,
                bounds.origin.y,
                bounds.size.width,
                bounds.size.height,
            ),
        });

        if only_active && is_not_active {
            break;
        }
    }

    windows
}

fn get_cf_boolean_value(dict: &CFDictionary, key: &str) -> bool {
    unsafe {
        match get_cf_dictionary_get_value::<CFBoolean>(dict, key) {
            Some(value) => CFBooleanGetValue(&*value),
            None => false,
        }
    }
}

fn get_cf_dictionary_get_value<T>(dict: &CFDictionary, key: &str) -> Option<*const T> {
    let key = CFString::from_str(key);
    let key_ref = key.as_ref() as *const CFString;
    if unsafe { CFDictionaryContainsKey(dict, key_ref.cast()) } {
        let value = unsafe { CFDictionaryGetValue(dict, key_ref.cast()) };
        Some(value as *const T)
    } else {
        None
    }
}

fn get_cf_number_value(dict: &CFDictionary, key: &str) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        match get_cf_dictionary_get_value::<CFNumber>(dict, key) {
            Some(number) => {
                CFNumberGetValue(
                    &*number,
                    CFNumberType::IntType,
                    &mut value as *mut _ as *mut c_void,
                );
                value
            }
            None => value,
        }
    }
}

fn get_cf_window_bounds_value(dict: &CFDictionary) -> Option<CGRect> {
    match get_cf_dictionary_get_value::<CFDictionary>(dict, "kCGWindowBounds") {
        Some(dict_react) => unsafe {
            let mut cg_rect = CGRect::default();
            if !dict_react.is_null()
                && CGRectMakeWithDictionaryRepresentation(Some(&*dict_react), &mut cg_rect)
            {
                Some(cg_rect as CGRect)
            } else {
                None
            }
        },
        None => None,
    }
}

fn get_bundle_identifier(app: &NSRunningApplication) -> String {
    unsafe {
        match app.bundleIdentifier() {
            Some(bundle_identifier) => bundle_identifier.to_string(),
            None => String::from(""),
        }
    }
}

fn get_cf_string_value(dict: &CFDictionary, key: &str) -> String {
    unsafe {
        match get_cf_dictionary_get_value::<CFString>(dict, key) {
            Some(value) => (*value).to_string(),
            None => String::from(""),
        }
    }
}
