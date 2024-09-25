use tauri::{Manager, WebviewWindowBuilder};

use crate::app::APP_HANDLE;

pub fn main_window_create() {
    let app_handle = APP_HANDLE.get();
    let app_handle = app_handle.unwrap();
    let builder = WebviewWindowBuilder::new(
        app_handle,
        "main",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .visible(false)
    .decorations(false)
    .skip_taskbar(false)
    .focused(true)
    .inner_size(0.0, 0.0)
    .resizable(false)
    ;

    _ = builder.build().unwrap();
}

pub fn main_window_crate_or_close() {
    let app_handle = APP_HANDLE.get();
    let app_handle = app_handle.unwrap();
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.close();
        return;
    }
    main_window_create()
}
