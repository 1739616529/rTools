use anyhow::{bail, Result};

use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{app::APP_HANDLE, window::create_main_window};

pub fn registry_hotkey(hotkey: &str, event: &str) -> Result<()> {
    let app_handle = APP_HANDLE.get();
    if app_handle.is_none() {
        bail!("failed to get the hotkey manager");
    }
    let app_handle = app_handle.unwrap();
    let manager = app_handle.global_shortcut();

    if manager.is_registered(hotkey) {
        manager.unregister(hotkey)?;
    };

    let func = match event.trim() {
        "open_main_window" => || {
            println!("hotkey open_main_window");
            create_main_window();
        },
        _ => bail!("invalid function \"{event}\""),
    };

    let _ = manager.on_shortcut(hotkey, move |_, _, event| {
        if let ShortcutState::Pressed = event.state {
            func();
        }
    });

    Ok(())
}
