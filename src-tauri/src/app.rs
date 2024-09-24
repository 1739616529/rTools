

use once_cell::sync::OnceCell;
use tauri::AppHandle;

use crate::init::current_setup;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            APP_HANDLE.get_or_init(|| app.handle().clone());


            #[cfg(all(desktop))]
            tauri::async_runtime::block_on(async move {
                current_setup(app).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet]);

    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        // tauri::RunEvent::WindowEvent { label, event, .. } => {
        //     if label == "main" {
        //         match event {
        //             tauri::WindowEvent::Destroyed => {
        //                 let _ = resolve::save_window_size_position(app_handle, true);
        //             }
        //             tauri::WindowEvent::CloseRequested { .. } => {
        //                 let _ = resolve::save_window_size_position(app_handle, true);
        //             }
        //             tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
        //                 let _ = resolve::save_window_size_position(app_handle, false);
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        _ => {}
    });

    println!("app started");
}
