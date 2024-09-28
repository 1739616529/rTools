use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use tauri::{plugin::TauriPlugin, AppHandle, Wry};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, ShortcutState, Shortcut, ShortcutEvent};

use super::window::open_main_window;



pub struct HotKey {
    code: Code,
    meta: bool,
    ctrl: bool,
    alt: bool,
    shift: bool,
}

impl HotKey {
    pub fn to_shortcut(&self) -> Shortcut{
        let mut modifiers = Modifiers::empty();

        if self.meta {
            modifiers |= Modifiers::META;
        }
        if self.ctrl {
            modifiers |= Modifiers::CONTROL;
        }
        if self.alt {
            modifiers |= Modifiers::ALT;
        }
        if self.shift {
            modifiers |= Modifiers::SHIFT;
        }

        Shortcut::new(Some(modifiers), self.code)
    }
}





pub type HotkeysState = Mutex<HotkeysStore>;

pub struct HotkeysStore {
    hotkeys: HashMap<String, HotKey>,
}


fn handle(app: &AppHandle, hotkey: &Shortcut, event: ShortcutEvent)  {

    if !matches!(event.state(), ShortcutState::Pressed) {
        return;
    }

    if "alt+Space" == hotkey.into_string() {
        _ = open_main_window(app);
    }


}


pub fn registry(app: &AppHandle) -> Result<()> {
    let global_shortcut = app.global_shortcut();

    let shortcut = HotKey {
        code: Code::Space,
        meta: false,
        ctrl: false,
        alt: true,
        shift: false,
    };
    global_shortcut.register(shortcut.to_shortcut())?;

    Ok(())
}


pub fn init() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(handle)
        .build()
}
