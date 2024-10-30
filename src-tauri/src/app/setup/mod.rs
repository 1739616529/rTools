use std::error::Error;

use anyhow::Result;
use tauri::{App, Manager};

use super::plugin::shortcut::{self, registry};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {

    // 接收错误 保存日志
    if let Err(err) = setup(app) {
        log::error!("{}", err);
    }
    Ok(())
}


fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    shortcut::setup(app.app_handle());
    registry(app.app_handle(), "Alt+Space".to_string(), "core:open.main.window".to_string())?;
    Ok(())
}
