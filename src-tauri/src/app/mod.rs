// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



mod setup;
mod plugin;

use tauri::{generate_context, Builder};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_window_state::StateFlags;


pub const AUTO_LAUNCH_ARG: &str = "--auto-launch";

pub struct Applican;

impl Applican {
    pub fn run() {

        #[cfg(target_os = "linux")]
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");


        let context = generate_context!();



        let app = Builder::default()
            .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
            .setup(setup::init)
            // 托盘
            .plugin(plugin::tray::init())
            // 快捷键
            .plugin(plugin::shortcut::init())
            // 开机启动
            .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![AUTO_LAUNCH_ARG])))
            // 日志信息
            // .plugin(
            //     tauri_plugin_log::Builder::default()
                // .targets([
                //     Target::new(TargetKind::LogDir { file_name: None }),
                //     Target::new(TargetKind::Webview),
                //     Target::new(TargetKind::Stderr),
                // ])
                // .level(log::LevelFilter::Info)
                // .build(),
            // )
            // 窗口记忆
            .plugin(
                tauri_plugin_window_state::Builder::default().with_state_flags(StateFlags::all()).build()
            )
            // .manage(state)
            .build(context).expect("error while running tauri application");


        app.run(|_app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });

    }

}
