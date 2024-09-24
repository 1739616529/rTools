use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Wry,
};

use crate::{app::APP_HANDLE, window::create_main_window};

pub fn create_tray() -> tauri::Result<()> {
    let app = APP_HANDLE.get().unwrap();

    let menu = create_menu(app)?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(handle_nemu_event)
        .on_tray_icon_event(|_, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                create_main_window();
            }
        })
        .build(app);

    Ok(())
}

pub fn create_menu(app: &AppHandle) -> tauri::Result<tauri::menu::Menu<Wry>> {
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

pub fn handle_nemu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
            std::process::exit(0);
        }
        _ => {}
    }
}
