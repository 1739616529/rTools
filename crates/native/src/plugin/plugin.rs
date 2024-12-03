use std::{ffi::{c_char, c_void, CString}, mem, path::PathBuf};

use crate::app::plugin::shortcut::registry;
use anyhow::Result;
use libloading::{library_filename, Library, Symbol};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::manifest::PluginManifest;

/// ##### plugin start state
///
/// Normal: default
///
/// Disabled: user disable
///
/// UnexpectedExitMultipleTimes: start error count
///
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub enum PluginStartState {
    Normal,
    Disabled,
    UnexpectedExitMultipleTimes(u32),
    Delete,
}


impl Default for PluginStartState {
    fn default() -> Self {
        Self::Normal
    }
}

type SendEvent = unsafe extern "C" fn(event: *const c_char) -> ();
type IpcFunCallback = extern "C" fn(*const c_char);

#[derive(Debug)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: PluginManifest,
    pub state: PluginStartState,
    pub dyn_lib: Library,
}

impl Plugin {
    pub fn build(manifest: PluginManifest, path: PathBuf, state: PluginStartState) -> Result<Self> {
        let mut pligin_path = path.clone();

        // set default main file name
        pligin_path.push(library_filename(
            manifest.main.as_ref().unwrap_or(&"plugin".to_string()),
        ));
        let dyn_lib = unsafe { libloading::Library::new(pligin_path)? };
        Ok(Self {
            path,
            manifest,
            state,
            dyn_lib,
        })
    }

    pub fn send(&self, event: &str) -> Result<()> {
        let dyn_lib = &self.dyn_lib;
        let event_fn: Symbol<SendEvent> = unsafe { dyn_lib.get(b"on_event") }?;
        unsafe {
            let str = CString::new(event).unwrap();
            event_fn(str.as_ptr());
        }
        Ok(())
    }
    pub fn load(&self, app: &AppHandle) -> Result<()> {
        if PluginStartState::Normal != self.state {
            return Ok(());
        }

        // registry plugins hotkey
        if self.manifest.shortcut.is_some() {
            self.manifest
                .shortcut
                .as_ref()
                .unwrap()
                .iter()
                .try_for_each(|shortcuts| -> Result<()> { shortcuts.registry(self, app) })?;
        }


        self.on_event(app);

        Ok(())
    }


    fn event_cb(&self, msg: &str) {


        println!("this is dyn lib send msg: {msg}")

    }

    fn on_event(&self, _app: &AppHandle) {
        if let Ok(event_fn) = unsafe { self.dyn_lib.get::<Symbol<extern "C" fn(extern "C" fn(*const c_char))>>(b"ffi_callback") } {
            println!("1");
            let event_cb = |msg: *const c_char| {
                let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
                self.event_cb(msg.to_str().unwrap());
            };
            // event_fn(event_cb);
            println!("3");
        };
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginShortcut {
    pub command: String,
    pub hotkey: String,
}

impl PluginShortcut {
    pub fn registry(&self, plugin: &Plugin, app: &AppHandle) -> Result<()> {

        registry(
            app,
            &self.hotkey,
            &format!(
                "plugin:hotkey:{}:{}",
                plugin.manifest.flag.as_str(),
                self.command.trim()
            ),
        )?;
        Ok(())
    }
}
