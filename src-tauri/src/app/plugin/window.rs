use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Result, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry,
};

pub const CORE_MAIN_WINDOW: &str = "core:window:main";
pub const CORE_SETTING_WINDOW: &str = "core:window:setting";
pub fn open_main_window(app: &AppHandle) -> Result<()> {
    let builder = WebviewWindowBuilder::new(app, CORE_MAIN_WINDOW, WebviewUrl::App("/".into()))
        .visible(false)
        .decorations(false)
        .skip_taskbar(false)
        .focused(true)
        .inner_size(800.0, 600.0)
        .resizable(false);
    builder.build()?;
    Ok(())
}
pub fn open_setting_window(app: &AppHandle) -> Result<()> {
    if match_window_to_default(app.get_webview_window(CORE_SETTING_WINDOW))? {
        return Ok(());
    }

    let builder =
        WebviewWindowBuilder::new(app, CORE_SETTING_WINDOW, WebviewUrl::App("/setting".into()))
            .visible(true)
            .decorations(true)
            .skip_taskbar(false)
            .focused(true)
            .title("设置")
            .inner_size(800.0, 600.0);

    builder.build()?;
    Ok(())
}

pub fn match_window_to_default(window: Option<WebviewWindow>) -> Result<bool> {
    match window {
        Some(window) => {
            window.unminimize()?;
            window.show()?;
            window.set_focus()?;
            return Ok(true);
        }
        None => Ok(false),
    }
}

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.window")
        .setup(|app, event| Ok(()))
        .build()
}
