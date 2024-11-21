pub mod plugin;
mod setup;


use tauri::{generate_context, Builder};
use tauri_plugin_autostart::MacosLauncher;

pub const AUTO_LAUNCH_ARG: &str = "--auto-launch";

pub struct Applican;

impl Applican {
    pub fn run() {
        #[cfg(target_os = "linux")]
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

        let mut log_level = log::LevelFilter::Info;
        #[cfg(debug_assertions)]
        {
            log_level = log::LevelFilter::Debug;
        }

        let context = generate_context!();

        let builder = Builder::default()
            .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
            // 托盘
            .plugin(plugin::tray::init())
            // 快捷键
            .plugin(plugin::shortcut::init())
            // 开机启动
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec![AUTO_LAUNCH_ARG]),
            ))
            // 日志
            .plugin(
                tauri_plugin_log::Builder::new()
                    .level(log_level)
                    .filter(|metadata| {
                        let target = metadata.target();
                        if
                                target == "tauri::app"
                            ||  target == "tauri::manager"
                            ||  target == "tracing::span"
                            ||  target == "wry::webview2"
                        {
                            return false
                        }
                        return true
                    })
                    .build()
                )
            // 窗口
            .plugin(plugin::window::init())
            // event
            .plugin(plugin::event::init())

            // 第三方 plugin
            .plugin(crate::plugin::init())


            .setup(setup::init)
            ;


        let app = builder
            .build(context)
            .expect("error while running tauri application");
        app.run(|_app, event| match event {
            _ => {}
        });
    }
}
