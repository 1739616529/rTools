use tauri::{AppHandle, Manager, WebviewWindowBuilder};

pub fn create_main_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }


    let builder = WebviewWindowBuilder::new(
        app_handle,
        "main",
        tauri::WebviewUrl::App("index.html".into())
    ).visible(false).decorations(false).skip_taskbar(false).inner_size(0.0, 0.0);

    _ = builder.build().unwrap();



}
