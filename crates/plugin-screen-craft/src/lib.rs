use plugin::{Config, Plugin};
use rtools_macro_plugin::rtools_plugin;


#[rtools_plugin]
pub struct PluginScreenCraft {
}

impl PluginScreenCraft {
    fn new(_config: &Config, _options: String) -> Self {
        Self{}
    }
}


impl Plugin for PluginScreenCraft {
    fn name(&self) -> &str {
        "pluginScreenCraft"
    }
}
