use std::error::Error;

use anyhow::Result;
use tauri::App;

pub fn init(_app: &mut App) -> Result<(), Box<dyn Error>> {
    Ok(())
}
