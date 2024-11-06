use std::sync::{Arc, Mutex};

pub mod manifest;
pub mod plugin;
pub mod plugins;
use anyhow::Result;
use plugins::Plugins;
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, RunEvent, Runtime, Wry,
};

pub const DEFAULT_FILENAME: &str = ".window-state.json";

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
