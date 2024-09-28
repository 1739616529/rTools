use tauri::{
    ipc::IpcResponse, plugin::{Builder, TauriPlugin}, AppHandle, LogicalPosition, LogicalSize, Manager, PhysicalPosition, PhysicalSize, Position, Result, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry
};

pub const CORE_MAIN_WINDOW: &str = "core:window:main";
pub const CORE_SETTING_WINDOW: &str = "core:window:setting";
pub fn open_main_window(app: &AppHandle) -> Result<()> {
    if app.get_webview_window(CORE_MAIN_WINDOW).is_some() {
        return  Ok(());;
    }
    let builder = WebviewWindowBuilder::new(app, CORE_MAIN_WINDOW, WebviewUrl::App("/".into()))
        .visible(true)
        .decorations(false)
        .skip_taskbar(false)
        .resizable(false)
        .transparent(true)
        .shadow(false)
        .always_on_top(true)
        .focused(true)
        ;
    let window = builder.build()?;
    if let Some(current_monitor) = window.current_monitor()? {
        let current_monitor_size = &current_monitor.size().to_logical::<f64>(current_monitor.scale_factor());
        let height = current_monitor_size.height;
        let width = current_monitor_size.width;
        window.set_position(Position::Logical(LogicalPosition::new(width / 3.0 ,height / 4.0)))?;
        window.set_size(LogicalSize::new(width / 3.0 , 56.0))?;




        // #[cfg(debug_assertions)]
        // window.open_devtools();
    }
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
