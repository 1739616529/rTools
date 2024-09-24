use once_cell::sync::OnceCell;
use tauri::App;

use crate::{hotkey::registry_hotkey, tray::create_tray};

pub static VERSION: OnceCell<String> = OnceCell::new();

pub async fn current_setup(app: &mut App) {
    let version = app.package_info().version.to_string();
    VERSION.get_or_init(|| version.clone());

    let _ = create_tray();
    let _ = registry_hotkey("CTRL+U", "open_main_window");
}
