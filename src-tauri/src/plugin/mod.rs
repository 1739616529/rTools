use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Deserialize;
use tokio::fs::{read_dir, read_to_string};

#[derive(Debug, Clone)]
pub enum PluginState {
    Normal,
    Disabled,
    UnexpectedExitMultipleTimes(u32),
}

pub struct Plugins {
    pub plugins: Arc<Mutex<Vec<Plugin>>>,
}

impl Plugins {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Plugins> = OnceCell::new();
        INSTANCE.get_or_init(|| Plugins {
            plugins: Arc::new(Mutex::new(vec![])),
        })
    }

    async fn load_plugin_info_by_local() -> Result<Vec<Plugin>> {
        let mut plug_path = read_dir("plugins").await?;

        let mut plugins = vec![];
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

            plugins.push(Plugin {
                path: entry.path(),
                manifest: manifest.clone(),
                state: PluginState::Normal,
            });
        }

        Ok(plugins)
    }

    pub async fn init(&self) -> Result<()> {
        *self.plugins.as_ref().lock() = Self::load_plugin_info_by_local().await?.into();
        println!("{:?}", self.plugins.as_ref().lock());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: PluginManifest,
    pub state: PluginState,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub flag: String,
    pub description: Option<String>,
    pub shortcut: Option<Vec<PluginShortcut>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginShortcut {
    pub command: String,
    pub hotkey: String,
}
