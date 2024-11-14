use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use tauri::{plugin::TauriPlugin, AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::plugin::AppHandleExt;

use super::{event::GlobalEventAppHandleExt, window::open_main_window};

struct ShortcutStateCache(Arc<Mutex<HashMap<String, String>>>);

fn handle(app: &AppHandle, hotkey: &Shortcut, event: ShortcutEvent) {
    if !matches!(event.state(), ShortcutState::Pressed) {
        return;
    }
    let binding = app.state::<ShortcutStateCache>().0.clone();
    let global_state = binding.lock().unwrap();
    let hotkey_str = hotkey.into_string();

    log::info!("hotkey event: {}", &hotkey_str);
    let hotkey_key = global_state.get(&hotkey_str);

    if hotkey_key.is_none() {
        return;
    }

    let hotkey_key = hotkey_key.unwrap().as_str();

    match hotkey_key {
        "core:hotkey:setup:open.main.window" => {
            let a = app.clone();
            tauri::async_runtime::spawn(async move {
                _ = open_main_window(&a);
            });
        }
        _ => {

            // 不使用广播 转发消息
            // let global_event = app.global_event();
            // global_event.send("plugin:hotkey", hotkey_key);

            let plugin_cache = app.global_plugin();
            let plugin_cache = plugin_cache.lock().unwrap();
            plugin_cache.send(hotkey_key);
        }
    }
}

pub fn registry(app: &AppHandle, shortcut: &str, event: &str) -> Result<()> {
    let global_shortcut = app.global_shortcut();

    // 解析快捷键
    let hotkey = Shortcut::try_from(shortcut)
        .map_err(|err| Error::msg(format!("hotkey parse error: {event} {err}",)))?;

    // 如果快捷键存在 抛出错误
    if global_shortcut.is_registered(hotkey) {
        return Err(Error::msg(format!(
            "registry shortcut {shortcut}[{event}] error, Already exists",
        )));
    }

    let binding = app.state::<ShortcutStateCache>().0.clone();
    let mut global_state = binding.lock().unwrap();
    let hotkey_str = hotkey.into_string();

    global_state.insert(hotkey_str.clone(), event.to_string());
    global_shortcut.register(hotkey)?;
    log::info!("registrt hotkey: {}[{event}]", &hotkey_str);
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
    let global_event = app.global_event();
    let app_handle = app.clone();
    global_event.on_mulit("registry_hotkey", Box::new(move|msg| {
        if let Err(err) = registry(&app_handle, msg[0], msg[1]) {
            log::error!("{}", err)
        }
    }));
}
