use std::any::Any;


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
    fn priority(&self) -> i32;
}
