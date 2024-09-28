use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::Result;
use libloading::{library_filename, Library};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Deserialize;
use tokio::fs::{read_dir, read_to_string};



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
    pub plugins: Arc<Mutex<HashMap<String, Plugin>>>,
}

impl Plugins {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Plugins> = OnceCell::new();
        INSTANCE.get_or_init(|| Plugins {
            plugins: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    pub async fn init(&self) -> Result<()> {
        *self.plugins.as_ref().lock() = Self::get_local_plugins_info().await?;
        self.plugin_exec();

        println!("{:?}", self.plugins.as_ref().lock());
        Ok(())
    }
    /// load local plugin info
    async fn get_local_plugins_info() -> Result<HashMap<String, Plugin>> {
        let mut plug_path = read_dir("plugins").await?;

        let mut plugins = HashMap::new();
        while let Some(entry) = plug_path.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let mut plug_dir = entry.path();
            plug_dir.push("manifest.json");
            if !plug_dir.exists() {
                continue;
            }

            let manifest = read_to_string(plug_dir).await?;

            let manifest = serde_json::from_str::<PluginManifest>(&manifest)?;

            let plugin = Plugin {
                path: entry.path(),
                manifest: manifest.clone(),
                state: PluginStartState::Normal,
            };

            plugins.insert(manifest.flag.to_string(), plugin);

        }

        Ok(plugins)
    }

    fn plugin_exec(&self) {
        for (_, plugin) in self.plugins.as_ref().lock().iter() {
            if let Some(err) = self.plugin_load(plugin).err() {
                println!("{:?}", err);
            }
        }
    }

    fn plugin_load(&self, plugin: &Plugin) -> Result<()> {
        // let lib = self.plugin_lib(plugin);

        if PluginStartState::Normal != plugin.state {
            return Ok(())
        }

        // registry plugins hotkey
        if plugin.manifest.shortcut.is_some() {
            let _ = plugin.manifest.shortcut.as_ref().unwrap().iter().map(|shortcuts| shortcuts.registry());
        }



        Ok(())

    }

    pub fn plugin_lib(&self, plugin: &Plugin) -> Result<Library> {
        let mut pligin_path = plugin.path.clone();

        // set default main file name
        pligin_path.push(library_filename(plugin.manifest.main.as_ref().unwrap_or(&"plugin".to_string())));
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
    pub fn registry (&self) -> Result<()>{
        // registry_hotkey(&self.hotkey, &self.command)?;
        Ok(())
    }
}
