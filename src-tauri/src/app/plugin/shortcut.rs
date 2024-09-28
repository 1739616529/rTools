use std::{collections::HashMap, sync::Mutex};

use tauri::{plugin::TauriPlugin, AppHandle, Wry};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent};



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



}



pub fn init() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(handle)
        .build()
}
