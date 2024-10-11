use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use tauri::{plugin::TauriPlugin, AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use super::window::open_main_window;

struct ShortcutStateCache(Arc<Mutex<HashMap<String, String>>>);

fn handle(app: &AppHandle, hotkey: &Shortcut, e: ShortcutEvent) {
    if !matches!(e.state(), ShortcutState::Pressed) {
        return;
    }
    let binding = app.state::<ShortcutStateCache>().0.clone();
    let global_state = binding.lock().unwrap();

    let event = global_state.get(&hotkey.into_string());

    if event.is_none() {
        return;
    }

    let event = event.unwrap().as_str();
    match event {
        "core:open.main.window" => {
            let a = app.clone();
            tauri::async_runtime::spawn(async move {
                _ = open_main_window(&a);
            });
        }
        _ => {
            println!("{event}")
        }
    }
}

pub fn registry(app: &AppHandle, shortcut: String, event: String) -> Result<()> {
    let global_shortcut = app.global_shortcut();
    let binding = app.state::<ShortcutStateCache>().0.clone();
    let mut global_state = binding.lock().unwrap();
    let _shortcut = shortcut.as_str();
    // 如果快捷键存在 抛出错误
    if global_shortcut.is_registered(_shortcut) {
        return Err(Error::msg("message"));
    }

    global_state.insert(shortcut.clone(), event);

    global_shortcut.register(_shortcut)?;

    Ok(())
}

pub fn init() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(handle)
        .build()
}
