use std::error::Error;

use anyhow::Result;
use tauri::{App, Manager};

use super::plugin::shortcut::registry;

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    registry(app.app_handle(), "Alt+Space".to_string(), "core:open.main.window".to_string())?;
    Ok(())
}
