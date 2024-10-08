mod plugin;
mod setup;


use tauri::{generate_context, Builder};
use tauri_plugin_autostart::MacosLauncher;

pub const AUTO_LAUNCH_ARG: &str = "--auto-launch";

pub struct Applican;

impl Applican {
    pub fn run() {
        #[cfg(target_os = "linux")]
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

        let context = generate_context!();

        let builder = Builder::default()
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
            .plugin(tauri_plugin_log::Builder::new().level(log::LevelFilter::Info).build())
            // 窗口
            .plugin(plugin::window::init())
            // .manage(state)
            ;


        let app = builder
            .build(context)
            .expect("error while running tauri application");
        app.run(|_app, event| match event {
            _ => {}
        });
    }
}
