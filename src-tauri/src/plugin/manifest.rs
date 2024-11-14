use serde::Deserialize;

use super::plugin::PluginShortcut;



#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub flag: String,
    pub description: Option<String>,
    pub shortcut: Option<Vec<PluginShortcut>>,
    pub main: Option<String>,
    pub permissions: Option<Vec<String>>
}
