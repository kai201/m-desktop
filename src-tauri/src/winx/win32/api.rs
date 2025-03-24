use std::{
    ffi::c_void,
    path::{Path, PathBuf},
};
use windows::{
    core::{w, Interface, HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::*,
        Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS},
        Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW},
        System::{Com::*, StationsAndDesktops::EnumDesktopWindows, Threading::*, Variant::VARIANT},
        UI::{Accessibility::*, WindowsAndMessaging::*},
    },
};

use crate::winx::core::{ActiveWindow, Api, WindowPosition};

#[derive(Debug)]
struct LangCodePage {
    pub w_language: u16,
    pub w_code_page: u16,
}

pub struct Win32API {}

impl Api for Win32API {
    fn get_active_window(&self) -> ActiveWindow {
        let active_window_hwnd = get_foreground_window();
        let active_window_position = get_window_position(active_window_hwnd).unwrap();
        let mut active_window_title = String::from("");
        let mut win_name = String::from("");
        let mut app_name = String::from("");
        let mut process_id: u32 = 0;
        unsafe { GetWindowThreadProcessId(active_window_hwnd, Some(&mut process_id as *mut u32)) };

        if let Ok(title) = get_window_title(active_window_hwnd) {
            active_window_title = title;
        }
        if let Ok(process_path) = get_process_path(process_id) {
            app_name = get_process_name(&process_path).unwrap();
        }

        if vec![String::from("微信"), String::from("WeChat")].contains(&app_name) {
            if let Ok(title) = get_window_title_wx(active_window_hwnd) {
                win_name = title;
            }
        }
        let active_window = ActiveWindow {
            title: active_window_title.clone(),
            app_name,
            position: active_window_position,
            process_id: process_id,
            window_id: format!("0x{:X}", active_window_hwnd.0 as usize),
            win_name,
            memory: 0,
        };
        active_window
    }

    fn get_windows(&self) -> Vec<ActiveWindow> {
        let mut windows: Vec<ActiveWindow> = Vec::new();

        enum_desktop_windows(|hwnd| {
            let active_window_position = get_window_position(hwnd).unwrap();
            let mut active_window_title = String::from("");

            if let Ok(window_title) = get_window_title(hwnd) {
                active_window_title = window_title;
            }

            let mut process_id: u32 = 0;
            unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id as *mut u32)) };

            let process_path = get_process_path(process_id).unwrap();
            let app_name = get_process_name(&process_path).unwrap();
            // format!("0x{:X}", hwnd.0 as usize)
            let active_window = ActiveWindow {
                title: active_window_title,
                app_name,
                position: active_window_position,
                process_id: process_id,
                window_id: format!("0x{:X}", hwnd.0 as usize),
                win_name: "None".to_string(),
                memory: 0,
            };
            if !(active_window.title.is_empty()
                && active_window.win_name.to_lowercase().eq(&"explorer"))
            {
                windows.push(active_window);
            }
            true
        });

        windows
    }

    fn activate(&self, window_id: String) {
        let cleaned_str = window_id.trim_start_matches("0x").trim_start_matches("0X");
        let h: usize = usize::from_str_radix(cleaned_str, 16).unwrap();
        let hwnd: HWND = HWND(h as *mut c_void);
        let is_valid = unsafe { IsWindow(Some(hwnd)) };
        if (is_valid).into() {}

        force_foreground_window(hwnd);
    }
}

/**
 * Get window title from HWND
 */
fn get_window_title(hwnd: HWND) -> Result<String, ()> {
    let title: String;
    unsafe {
        let mut v: Vec<u16> = vec![0; 255];
        let title_len = GetWindowTextW(hwnd, &mut v);
        title = String::from_utf16_lossy(&v[0..(title_len as usize)]);
    };
    Ok(title)
}

fn get_window_title_wx(hwnd: HWND) -> windows::core::Result<String> {
    unsafe {
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let element: IUIAutomationElement = automation.ElementFromHandle(hwnd)?;

        let condtion =
            automation.CreatePropertyCondition(UIA_NamePropertyId, &VARIANT::from("会话"))?;

        let root_el = element.FindFirst(TreeScope_Descendants, &condtion)?;
        let elements = root_el.FindAll(TreeScope_Children, &automation.CreateTrueCondition()?)?;
        let count = elements.Length()?;

        for i in 0..count {
            let elem: IUIAutomationElement = elements.GetElement(i)?;

            let title: windows::core::BSTR = elem.CurrentName()?;

            let has: IUIAutomationSelectionItemPattern =
                elem.GetCurrentPattern(UIA_SelectionItemPatternId)?.cast()?;

            if has.CurrentIsSelected()?.as_bool() {
                let title_str = title.to_string();
                return Ok(title_str);
            }
        }
    }

    Ok(String::new())
}

fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

fn force_foreground_window(hwnd: HWND) {
    unsafe {
        let current_thread_id = GetCurrentThreadId();
        let target_thread_id = GetWindowThreadProcessId(hwnd, None);

        if current_thread_id != target_thread_id {
            let _ = AttachThreadInput(current_thread_id, target_thread_id, TRUE.into());
            let _ = SetForegroundWindow(hwnd);
            let _ = AttachThreadInput(current_thread_id, target_thread_id, FALSE.into());
        } else {
            let _ = SetForegroundWindow(hwnd);
        }

        if windows::Win32::UI::WindowsAndMessaging::IsIconic(hwnd) == TRUE {
            let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::SW_RESTORE,
            );
        }
    }
}

fn get_window_position(hwnd: HWND) -> Result<WindowPosition, ()> {
    unsafe {
        let mut rect: RECT = std::mem::zeroed();

        // Try DwmGetWindowAttribute first for more accurate bounds
        let result = DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut RECT as *mut _,
            std::mem::size_of::<RECT>() as u32,
        );

        // Fall back to GetWindowRect if DwmGetWindowAttribute fails
        if result.is_err() && !GetWindowRect(hwnd, &mut rect).is_ok() {
            return Ok(Default::default());
        }

        Ok(from_win_rect(&rect))
    }
}

fn get_process_path(process_id: u32) -> Result<PathBuf, ()> {
    let process_handle = get_process_handle(process_id)?;
    let mut lpdw_size: u32 = MAX_PATH;
    let mut process_path_raw = vec![0; MAX_PATH as usize];
    let process_path_pwstr = PWSTR::from_raw(process_path_raw.as_mut_ptr());

    let process_path = unsafe {
        let success = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_WIN32,
            process_path_pwstr,
            &mut lpdw_size,
        );

        close_process_handle(process_handle);

        if !success.is_ok() {
            return Err(());
        }

        process_path_pwstr.to_string().map_err(|_| ())?
    };

    Ok(Path::new(&process_path).to_path_buf())
}

fn get_process_name(process_path: &Path) -> Result<String, ()> {
    let file_description = get_file_description(process_path);
    if file_description.is_ok() && !file_description.as_ref().unwrap().is_empty() {
        return file_description;
    }

    let process_file_name = process_path
        .file_stem()
        .unwrap_or(std::ffi::OsStr::new(""))
        .to_str()
        .unwrap_or("")
        .to_owned();

    Ok(process_file_name)
}

fn get_file_description(process_path: &Path) -> Result<String, ()> {
    let process_path_hstring: HSTRING = process_path.as_os_str().into();

    let info_size = unsafe { GetFileVersionInfoSizeW(&process_path_hstring, None) };

    if info_size == 0 {
        return Err(());
    }

    let mut file_version_info = vec![0u8; info_size.try_into().unwrap()];

    let file_info_query_success = unsafe {
        GetFileVersionInfoW(
            &process_path_hstring,
            Some(0),
            info_size,
            file_version_info.as_mut_ptr().cast(),
        )
    };
    if !file_info_query_success.is_ok() {
        return Err(());
    }

    let mut lang_ptr = std::ptr::null_mut();
    let mut len = 0;
    let lang_query_success = unsafe {
        VerQueryValueW(
            file_version_info.as_ptr().cast(),
            w!("\\VarFileInfo\\Translation"),
            &mut lang_ptr,
            &mut len,
        )
    };
    if !lang_query_success.as_bool() {
        return Err(());
    }

    let lang: &[LangCodePage] =
        unsafe { std::slice::from_raw_parts(lang_ptr as *const LangCodePage, 1) };

    if lang.is_empty() {
        return Err(());
    }

    let mut query_len: u32 = 0;

    let lang = lang.first().unwrap();
    let lang_code = format!(
        "\\StringFileInfo\\{:04x}{:04x}\\FileDescription",
        lang.w_language, lang.w_code_page
    );
    let lang_code = PCWSTR(HSTRING::from(&lang_code).as_ptr());

    let mut file_description_ptr = std::ptr::null_mut();

    let file_description_query_success = unsafe {
        VerQueryValueW(
            file_version_info.as_ptr().cast(),
            lang_code,
            &mut file_description_ptr,
            &mut query_len,
        )
    };

    if !file_description_query_success.as_bool() {
        return Err(());
    }

    let file_description =
        unsafe { std::slice::from_raw_parts(file_description_ptr.cast(), query_len as usize) };
    let file_description = String::from_utf16_lossy(file_description);
    let file_description = file_description.trim_matches(char::from(0)).to_owned();

    Ok(file_description)
}

fn from_win_rect(rect: &RECT) -> WindowPosition {
    WindowPosition {
        x: rect.left as f64,
        y: rect.top as f64,
        width: (rect.right - rect.left) as f64,
        height: (rect.bottom - rect.top) as f64,
    }
}

fn get_process_handle(process_id: u32) -> Result<HANDLE, ()> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) };
    handle.map_err(|_| ())
}

fn close_process_handle(process_handle: HANDLE) {
    let _ = unsafe { CloseHandle(process_handle) };
}

/** Function with callback as parameter to get open windows */
fn enum_desktop_windows<Callback: FnMut(HWND) -> bool>(callback: Callback) {
    unsafe {
        let lparam = LPARAM(&callback as *const _ as isize);
        let _ = EnumDesktopWindows(None, Some(enum_desktop_windows_proc::<Callback>), lparam);
    }
}

/** Functions for callback */
unsafe extern "system" fn enum_desktop_windows_proc<Callback: FnMut(HWND) -> bool>(
    hwnd: HWND,
    lparam: LPARAM,
) -> BOOL {
    let callback = lparam.0 as *mut Callback;
    unsafe {
        if IsWindow(Some(hwnd)).as_bool() && IsWindowVisible(hwnd).as_bool() {
            // If problem with callback stop loop
            if !((*callback)(hwnd)) {
                return FALSE;
            }
        }

        TRUE
    }
}
