use std::collections::HashMap;

use plugin::{Config, Plugin};
use rtools_macro_plugin::rtools_plugin;

#[rtools_plugin]
pub struct PluginScreenCraft {
    pub verison: &'static str,
}

impl PluginScreenCraft {
    fn new(_config: &Config, _options: String) -> Self {
        Self {
            verison: env!("CARGO_PKG_VERSION")
        }
    }
}

impl Plugin for PluginScreenCraft {
    fn name(&self) -> &str {
        "pluginScreenCraft"
    }
    fn shortcut(&self) -> HashMap<&str, &str> {
        HashMap::from([("start", "Ctrl+Alt+K")])
    }
    fn on_shortcut(&self, key: &str) {
        println!("shortcut key is: {key}")
    }
    fn verison(&self) -> &str {
        self.verison
    }
}
