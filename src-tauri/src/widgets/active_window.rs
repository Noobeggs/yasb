use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use ts_rs::TS;
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GetForegroundWindow};

use crate::win32::utils;

use super::base::WidgetCallbacks;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/widget/active_window/")]
pub struct IgnoreLists {
    class: Option<Vec<String>>,
    process: Option<Vec<String>>,
    title: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/widget/active_window/")]
pub struct ActiveWindowWidgetProps {
    class: Option<String>,
    label: Option<String>,
    label_alt: Option<String>,
    label_tooltip: Option<String>,
    exclusive: Option<bool>,
    ignore: Option<IgnoreLists>,
    callbacks: Option<WidgetCallbacks>,
}

impl Default for ActiveWindowWidgetProps {
    fn default() -> ActiveWindowWidgetProps {
        ActiveWindowWidgetProps {
            class: None,
            label: None,
            label_alt: None,
            label_tooltip: None,
            exclusive: None,
            ignore: None,
            callbacks: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/widget/active_window/")]
pub struct WindowProcessInfo {
    pid: u32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/widget/active_window/")]
pub struct ActiveWindowPayload {
    hwnd: isize,
    handle: Option<isize>,
    pid: u32,
    tid: u32,
    title: Option<String>,
    class: Option<String>,
    exe_path: Option<String>,
    exe_name: Option<String>,
    monitor: Option<String>,
}

pub fn handle_window_title_change(app_handle: &AppHandle, hwnd: HWND) {
    let (process_id, thread_id) = utils::get_thread_process_id(hwnd.clone());
    let proc_handle = utils::get_pid_handle(process_id.clone());
    let proc_path = if proc_handle.is_some() {
        utils::get_exe_path(proc_handle.clone().unwrap())
    } else {
        None
    };
    let proc_name = if proc_path.is_some() {
        utils::get_exe_name_from_path(proc_path.clone().unwrap())
    } else {
        None
    };
    let monitor_info = utils::get_monitor_info(hwnd.clone());
    let monitor_name = utils::get_monitor_name(monitor_info);

    let payload = ActiveWindowPayload {
        hwnd: hwnd.0 as isize,
        handle: if proc_handle.is_some() {
            Some(proc_handle.unwrap().0)
        } else {
            None
        },
        pid: process_id,
        tid: thread_id,
        title: utils::get_win_title(hwnd.clone()),
        class: utils::get_win_class(hwnd.clone()),
        exe_path: proc_path,
        exe_name: proc_name,
        monitor: monitor_name,
    };
    let _ = app_handle.emit_all("ActiveWindowChanged", payload);
}

#[tauri::command]
pub fn detect_foreground_window(app_handle: tauri::AppHandle) {
    let foreground_hwnd = unsafe { GetForegroundWindow() };
    handle_window_title_change(&app_handle, foreground_hwnd);
}