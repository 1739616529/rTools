use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use tauri::{plugin::TauriPlugin, AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use super::window::open_main_window;

struct ShortcutStateCache(Arc<Mutex<HashMap<String, String>>>);

fn handle(app: &AppHandle, hotkey: &Shortcut, event: ShortcutEvent) {

    if !matches!(event.state(), ShortcutState::Pressed) {
        return;
    }
    let binding = app.state::<ShortcutStateCache>().0.clone();
    let global_state = binding.lock().unwrap();

    let hotkey_key = global_state.get(&hotkey.into_string().to_lowercase());

    if hotkey_key.is_none() {
        return;
    }

    let hotkey_key = hotkey_key.unwrap().as_str();
    match hotkey_key {
        "core:open.main.window" => {
            let a = app.clone();
            tauri::async_runtime::spawn(async move {
                _ = open_main_window(&a);
            });
        }
        _ => {
            println!("{hotkey_key}")
        }
    }
}

pub fn registry(app: &AppHandle, shortcut: String, event: String) -> Result<()> {
    let global_shortcut = app.global_shortcut();
    let _shortcut = shortcut.as_str();
    // 如果快捷键存在 抛出错误
    if global_shortcut.is_registered(_shortcut) {
        return Err(Error::msg(format!("registry shortcut {} error, Already exists", _shortcut)));
    }



    let binding = app.state::<ShortcutStateCache>().0.clone();
    let mut global_state = binding.lock().unwrap();



    global_state.insert(shortcut.to_lowercase().clone(), event);

    global_shortcut.register(_shortcut)?;


    Ok(())
}

pub fn init() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(handle)
        .build()
}


pub fn setup(app: &AppHandle) {
    let shortcut = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    app.manage(ShortcutStateCache(shortcut));
}
