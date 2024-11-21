use std::sync::{Arc, Mutex};

pub mod manifest;
pub mod plugin;
pub mod plugins;
use anyhow::Result;
use plugins::Plugins;
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, RunEvent, Wry,
};

pub const DEFAULT_FILENAME: &str = ".window-state.json";

pub trait AppHandleExt {
    fn global_plugin(&self) -> Arc<Mutex<Plugins>>;
    /// Saves all open windows state to disk
    fn save_plugins_state(&self) -> Result<()>;
    fn load_plugins(&self);
}

impl AppHandleExt for AppHandle {
    // 保存插件信息到本地
    fn save_plugins_state(&self) -> Result<()> {
        Ok(())
    }

    // 加载第三方插件
    fn load_plugins(&self) {
        let cache = self.state::<PluginStateCache>();
        let mut state = cache.0.lock().unwrap();
        let app = self.app_handle();
        state.get_local_plugins_info(app);
        if let Err(err) = state.load_local_plugins(app) {
            log::error!("{}", err);
        }

        // let global_event = self.global_event();
        // global_event.on("plugin:hotkey", Box::new(|e| {
        //     let cache = self.state::<PluginStateCache>();
        //     let mut state = cache.0.lock().unwrap();
        //     state.send(event)
        // }));
    }

    fn global_plugin(&self) -> Arc<Mutex<Plugins>> {
        self.state::<PluginStateCache>().inner().0.clone()
    }
}

pub struct PluginStateCache(Arc<Mutex<Plugins>>);
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
