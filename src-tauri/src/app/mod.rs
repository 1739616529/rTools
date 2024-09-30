

mod plugin;
mod setup;

use tauri::{generate_context, Builder};
use tauri_plugin_autostart::MacosLauncher;
// use tauri_plugin_log::{Target, TargetKind};

pub const AUTO_LAUNCH_ARG: &str = "--auto-launch";

pub struct Applican;

impl Applican {
    pub fn run() {
        #[cfg(target_os = "linux")]
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

        let context = generate_context!();

        let mut builder = Builder::default()
            .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
            .setup(setup::init)
            // 托盘
            .plugin(plugin::tray::init())
            // 快捷键
            .plugin(plugin::shortcut::init())
            // 开机启动
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec![AUTO_LAUNCH_ARG]),
            ))
            // 窗口
            .plugin(plugin::window::init())
            // 日志信息
            .plugin(
                tauri_plugin_log::Builder::new()
                    .build(),
            )
            // .manage(state)
            ;



        #[cfg(debug_assertions)]
        {
            let devtools = tauri_plugin_devtools::init(); // initialize the plugin as early as possible
            builder = builder.plugin(devtools);
        }

        let app = builder.build(context)
            .expect("error while running tauri application");
        app.run(|_app, event| match event {
            _ => {}
        });
    }
}
