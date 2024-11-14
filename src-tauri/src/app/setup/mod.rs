use std::error::Error;

use anyhow::Result;
use tauri::{App, Manager};
use crate::plugin::AppHandleExt;

use super::plugin::shortcut::{self, registry};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {

    // 接收错误 保存日志
    if let Err(err) = setup(app) {
        log::error!("{}", err);
    }
    Ok(())
}


fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.app_handle();
    shortcut::setup(app_handle);
    registry(app_handle, "Alt+Space", "core:setup:open.main.window")?;

    // 加载 第三方插件
    app_handle.load_plugins();
    Ok(())
}
