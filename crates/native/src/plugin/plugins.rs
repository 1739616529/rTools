use std::{
    collections::HashMap, env, fs::{read_dir, read_to_string}, io
};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{plugin::manifest::PluginManifest, util::event_match};

use super::{
    plugin::{Plugin, PluginStartState},
    DEFAULT_FILENAME,
};

pub struct Plugins {
    pub plugins: HashMap<String, Plugin>,
    pub info: HashMap<String, PluginsInfo>,
}

impl Plugins {
    pub fn init() -> Self {
        Self {
            plugins: Default::default(),
            info: Default::default(),
        }
    }
    // 获取本地插件信息
    pub fn get_local_plugins_info(&mut self, app: &AppHandle) {
        let app_dir = app.path().app_config_dir();
        if app_dir.is_err() {
            self.info = Default::default();
            return;
        }

        let state_path = app_dir.unwrap().join(&DEFAULT_FILENAME.to_string());

        if !state_path.exists() {
            self.info = Default::default();
            return;
        }
        self.info = std::fs::read(state_path)
            .map_err(Error::from)
            .and_then(|state| serde_json::from_slice(&state).map_err(Into::into))
            .unwrap_or_default();


    }

    // 注册插件
    pub fn plugins_registry(&mut self) -> Result<()> {

        let mut plug_path = read_dir("../plugins").map_err(|err| {
            io::Error::new(err.kind(), format!("read plugins dir error: {err}"))
        })?;
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
            let plugin_info = self.info.entry(manifest.flag.clone()).or_insert_with(Default::default);

            let plugin = Plugin::build(manifest.clone(), entry.path(), plugin_info.state.clone())?;
            log::info!("plugins registry: {}  [state]: {:?}", &manifest.flag, &plugin.state);
            self.plugins.insert(manifest.flag.clone(), plugin);
            Ok(())
        })?;

        Ok(())
    }

    pub fn load_local_plugins(&mut self, app: &AppHandle) -> Result<()> {
        self.plugins_registry()?;
        self.plugins.iter_mut().for_each(| (_, plugin)| {
            if let Err(err) = plugin.load(app) {
                log::error!("load plugin error: {} --> {}", plugin.manifest.flag.as_str(), err);
            }
        });
        Ok(())
    }
    pub fn send(&self, event: &str) {
        let (module, _, flag, _ ) = event_match(event).unwrap();
        if module != "plugin" {
            return
        }
        let plugin = self.plugins.get(flag).unwrap();
        if let Err(err) = plugin.send(event) {
            log::error!("{}", err);
        };
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PluginsInfo {
    pub state: PluginStartState,
    pub flag: String,
}

impl Default for PluginsInfo {
    fn default() -> Self {
        Self {
            state: PluginStartState::Normal,
            flag: "".to_string(),
        }
    }
}
