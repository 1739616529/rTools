use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use libloading::{library_filename, Library};
use serde::Deserialize;
use tauri::{
    plugin::{Builder, TauriPlugin}, AppHandle, Manager, RunEvent, Runtime, Wry
};

pub const DEFAULT_FILENAME: &str = ".window-state.json";

/// ##### plugin start state
///
/// Normal: default
///
/// Disabled: user disable
///
/// UnexpectedExitMultipleTimes: start error count
///
#[derive(Debug, Clone, PartialEq)]
pub enum PluginStartState {
    Normal,
    Disabled,
    UnexpectedExitMultipleTimes(u32),
}

pub struct Plugins {
    pub plugins: HashMap<String, Plugin>
}

impl Plugins {
    pub fn init() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    /// load local plugin info
    fn get_local_plugins_info(&mut self) -> Result<()> {
        log::info!()
        let mut plug_path: std::fs::ReadDir = read_dir("plugins")?;
        plug_path.try_for_each(|entry| -> Result<(), Error> {
            let entry = entry?;

            // 如果不是文件夹
            if !entry.file_type()?.is_dir() {
                return Ok(());
            }

            let mut plugin_dir = entry.path();

            plugin_dir.push("manifest.json");

            // 如果 manifest.json 不存在
            if !plugin_dir.exists() {
                return Ok(());
            }

            let manifest = read_to_string(plugin_dir)?;
            let manifest = serde_json::from_str::<PluginManifest>(&manifest)?;
            let plugin = Plugin {
                path: entry.path(),
                manifest: manifest.clone(),
                state: PluginStartState::Normal,
            };
            self.plugins.insert(manifest.flag.to_string(), plugin);

            Ok(())
        })?;

        Ok(())
    }

    fn send(message: String) {

    }

    fn plugin_load(&self, plugin: &Plugin) -> Result<()> {
        // let lib = self.plugin_lib(plugin);

        if PluginStartState::Normal != plugin.state {
            return Ok(());
        }

        // registry plugins hotkey
        if plugin.manifest.shortcut.is_some() {
            let _ = plugin
                .manifest
                .shortcut
                .as_ref()
                .unwrap()
                .iter()
                .map(|shortcuts| shortcuts.registry());
        }

        Ok(())
    }

    pub fn plugin_lib(&self, plugin: &Plugin) -> Result<Library> {
        let mut pligin_path = plugin.path.clone();

        // set default main file name
        pligin_path.push(library_filename(
            plugin
                .manifest
                .main
                .as_ref()
                .unwrap_or(&"plugin".to_string()),
        ));
        unsafe {
            let lib = libloading::Library::new(pligin_path)?;
            Ok(lib)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: PluginManifest,
    pub state: PluginStartState,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub flag: String,
    pub description: Option<String>,
    pub shortcut: Option<Vec<PluginShortcut>>,
    pub main: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginShortcut {
    pub command: String,
    pub hotkey: String,
}

impl PluginShortcut {
    pub fn registry(&self) -> Result<()> {
        // registry_hotkey(&self.hotkey, &self.command)?;
        Ok(())
    }
}


pub trait AppHandleExt {
    /// Saves all open windows state to disk
    fn save_plugins_state(&self) -> Result<()>;
    fn load_plugins(&self) -> Result<()>;
}

impl<R: Runtime> AppHandleExt for AppHandle<R> {

    // 保存插件信息到本地
    fn save_plugins_state(&self) -> Result<()> {
        Ok(())
    }


    // 加载第三方插件
    fn load_plugins(&self) -> Result<()> {
        let cache = self.state::<PluginStateCache>();
        let mut state = cache.0.lock().unwrap();
        state.get_local_plugins_info()?;
        Ok(())
    }
}

struct PluginStateCache(Arc<Mutex<Plugins>>);
pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.plugin")
        .setup(|app, _| {


            let plugins = Plugins::init();
            app.manage(PluginStateCache(Arc::new(Mutex::new(plugins))));



            Ok(())
        })



        .on_event(|app, e| match e {



            RunEvent::ExitRequested { .. } => {
                _ = app.save_plugins_state();
            }

            _ => {}
        })
        .build()
}
