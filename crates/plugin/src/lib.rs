use anyhow::{Error, Result};
pub use libloading;
use libloading::{Library, Symbol};
pub use serde;
pub use serde_json;
use std::{any::Any, collections::HashMap, ffi::OsStr, sync::Arc};

pub const DEFAULT_PRIORITY: i32 = 100;
pub trait Plugin: Any + Send + Sync {
    /// plugin name
    ///
    /// # Example
    /// ```rust
    /// struct PluginTest {}
    ///
    /// impl Plugin for PluginTest {
    ///     fn name(&self) -> &str {
    ///         "PluginTest"
    ///     }
    /// }
    /// ```
    fn name(&self) -> &str;

    /// 注册快捷键
    /// # Example
    /// ```
    /// struct PluginTest {}
    ///
    /// impl Plugin for PluginTest {
    ///     fn shortcut(&self) -> HashMap<&str, &str> {
    ///         HashMap::from([
    ///             ("open", "Ctrl+Alt+K")
    ///         ])
    ///     }
    /// }
    /// ```
    fn shortcut(&self) -> HashMap<&str, &str> {
        Default::default()
    }

    /// 快捷键触发回调
    /// # Example
    /// ```
    /// struct PluginTest {}
    /// impl Plugin for PluginTest {
    ///     fn on_shortcut(&self, key: &str) {
    ///        println!("shortcut open {key}")
    ///     }
    /// }
    /// ```
    fn on_shortcut(&self, key: &str) {
        let _ = key;
    }

    /// plugin priority
    ///
    /// 优先执行权限 越小 优先级越高
    ///
    /// # Example
    /// ```rust
    /// struct PluginTest {}
    ///
    /// impl Plugin for PluginTest {
    ///     fn priority(&self) -> i32 {
    ///         1
    ///     }
    /// }
    /// ```
    fn priority(&self) -> i32 {
        DEFAULT_PRIORITY
    }

    /// plugin version
    ///
    /// 插件版本号
    ///
    /// # Example
    /// ```rust
    /// struct PluginTest {}
    ///
    /// impl Plugin for PluginTest {
    ///     fn verison(&self) -> str {
    ///         self.version
    ///     }
    /// }
    /// ```
    fn verison(&self) -> &str;

    /// plugin version
    ///
    /// 插件系统版本号 用来对比版本 进行迭代
    ///
    /// # Example
    /// ```rust
    /// struct PluginTest {}
    ///
    /// impl Plugin for PluginTest {
    ///     fn plugin_verison(&self) -> str {
    ///         self.version
    ///     }
    /// }
    /// ```
    fn plugin_verison(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

pub struct Config {
    pub name: String,
}

pub unsafe fn plugin_loader<P: AsRef<OsStr> + std::fmt::Display>(
    filename: P,
    config: &Config,
) -> Result<(Arc<dyn Plugin>, Library)> {
    type PluginCreate = fn(config: &Config) -> Arc<dyn Plugin>;
    let lib = Library::new(filename.as_ref())?;
    let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
    let plugin = constructor(config);
    Ok((plugin, lib))
}

pub struct PluginAdapter {
    /// plugin instance
    pub plugin: Arc<dyn Plugin>,
    /// dynamic lib of this plugin, this lib should created and destroyed with the plugin instance as the same time
    pub _lib: Library,
}

impl PluginAdapter {
    pub fn new(plugin_path: &String, config: &Config) -> Result<Self> {
        let (plugin, _lib) = unsafe {
            plugin_loader(plugin_path, config)
                .map_err(|e| Error::msg(format!("Load rust plugin {plugin_path} failed. {e:?}")))?
        };
        Ok(Self { plugin, _lib })
    }
}
