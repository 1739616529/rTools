use tauri::{Manager, WebviewWindowBuilder};

use crate::app::APP_HANDLE;

pub fn create_main_window() {
    let app_handle = APP_HANDLE.get();
    let app_handle = app_handle.unwrap();

    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.close();

        println!("main window already exists");
        return;
    }

    let builder = WebviewWindowBuilder::new(
        app_handle,
        "main",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .visible(false)
    .decorations(false)
    .skip_taskbar(false)
    .focused(true)
    .inner_size(0.0, 0.0);

    _ = builder.build().unwrap();
}
