use tauri::{menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, plugin::{Builder, TauriPlugin}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Manager, Wry};

use super::window::{open_main_window, open_setting_window, CORE_MAIN_WINDOW};



fn build_tray_menu(app: &AppHandle) -> tauri::Result<tauri::menu::Menu<Wry>> {
    let quit_app = &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let divid_menu_item = &PredefinedMenuItem::separator(app).unwrap();

    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "setting", "Setting", true, None::<&str>)?,
            divid_menu_item,
            quit_app,
        ],
    )?;

    Ok(menu)
}



fn build_tray(app: &AppHandle)  -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(handle_nemu_event)
        .on_tray_icon_event(|app, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app_handle =  app.app_handle();
                match app_handle.get_webview_window(CORE_MAIN_WINDOW) {
                    Some(window) => {_ = window.close();},
                    None => {_ = open_main_window(app_handle);},
                };

            }
        })
        .build(app);

    Ok(())
}



fn handle_nemu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
            std::process::exit(0);
        }
        "setting" => {
            _ = open_setting_window(app);
        }
        _ => {}
    }
}






pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.tray")
        .setup(|app, event| {
            build_tray(app)?;
            Ok(())
        })
    .build()
}
