use tauri::{menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, plugin::{Builder, TauriPlugin}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Wry};

use super::window::{open_main_window, open_setting_window};



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
                _ = open_main_window(app_handle);
            }
        })
        .build(app);

    Ok(())
}



fn handle_nemu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "setting" => {
            _ = open_setting_window(app);
        }
        _ => {}
    }
}






pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.tray")
        .setup(|app, _| {
            build_tray(app)?;
            Ok(())
        })
        .on_event(|_, e| match e {
            tauri::RunEvent::ExitRequested { api, code , .. } => {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
            _ => {}
        })
    .build()
}
