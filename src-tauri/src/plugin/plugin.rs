use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use super::{manifest::PluginManifest, plugins::PluginStartState};


#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: PluginManifest,
    pub state: PluginStartState,
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
