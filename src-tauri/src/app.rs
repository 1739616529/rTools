

use crate::tray::create_tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        #[cfg(all(desktop))]
        {
            let handle = app.handle();
            create_tray(handle)?;
        }
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
