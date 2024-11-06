use std::{collections::HashMap, fs::{read_dir, read_to_string}};

use anyhow::{Error, Result};
use libloading::{library_filename, Library};

use crate::plugin::manifest::PluginManifest;

use super::plugin::Plugin;


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
    pub fn get_local_plugins_info(&mut self) -> Result<()> {
        log::info!("321321");
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
