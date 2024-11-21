use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    sync::{Arc, Mutex},
};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin}, AppHandle, LogicalPosition, LogicalSize, Manager, Result, RunEvent, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, WindowEvent, Wry
};

pub const CORE_MAIN_WINDOW: &str = "core:window:main";
pub const CORE_SETTING_WINDOW: &str = "core:window:setting";
pub const DEFAULT_FILENAME: &str = ".window-state.json";
pub fn open_main_window(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window(CORE_MAIN_WINDOW) {
        window.close()?;
        return Ok(());
    }
    let builder = WebviewWindowBuilder::new(app, CORE_MAIN_WINDOW, WebviewUrl::App("/".into()))
        .visible(false)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(false)
        .transparent(true)
        .always_on_top(true)
        .title("main.window")

        // .on_page_load(|window, payload| {
        //     if matches!(payload.event(), PageLoadEvent::Started) {
        //         _ = window.show();
        //     }
        // })
        ;
    builder.build()?;

    Ok(())
}
pub fn open_setting_window(app: &AppHandle) -> Result<()> {
    if match_window_to_default(app.get_webview_window(CORE_SETTING_WINDOW))? {
        return Ok(());
    }
    let builder =
        WebviewWindowBuilder::new(app, CORE_SETTING_WINDOW, WebviewUrl::App("/setting".into()))
            .visible(true)
            .decorations(true)
            .title("设置")
            .inner_size(800.0, 600.0);

    builder.build()?;
    Ok(())
}

pub fn match_window_to_default(window: Option<WebviewWindow>) -> Result<bool> {
    match window {
        Some(window) => {
            window.unminimize()?;
            window.show()?;
            window.set_focus()?;
            return Ok(true);
        }
        None => Ok(false),
    }
}

pub trait AppHandleExt {
    /// Saves all open windows state to disk
    fn save_window_state(&self) -> Result<()>;
}

impl<R: Runtime> AppHandleExt for tauri::AppHandle<R> {
    fn save_window_state(&self) -> Result<()> {
        let app_dir = self.path().app_config_dir()?;
        let config_dir = app_dir.join(&DEFAULT_FILENAME.to_string());


        let cache = self.state::<WindowStateCache>();
        let cache = cache.0.lock().unwrap();

        create_dir_all(&app_dir)
            .and_then(|_| File::create(&config_dir).map_err(Into::into))
            .and_then(|mut f| serde_json::to_writer_pretty(&mut f, &*cache).map_err(Into::into))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct WindowState {
    width: u32,
    height: u32,
    x: f64,
    y: f64,
    prev_x: i32,
    prev_y: i32,
    maximized: bool,
    visible: bool,
    decorated: bool,
    fullscreen: bool,
}

pub struct WindowStateCache(Arc<Mutex<HashMap<String, WindowState>>>);

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            x: Default::default(),
            y: Default::default(),
            prev_x: Default::default(),
            prev_y: Default::default(),
            maximized: Default::default(),
            visible: true,
            decorated: true,
            fullscreen: Default::default(),
        }
    }
}

fn get_cache_config(app: &AppHandle) -> Arc<Mutex<HashMap<String, WindowState>>> {
    if let Ok(app_dir) = app.path().app_config_dir() {
        let state_path = app_dir.join(&DEFAULT_FILENAME.to_string());
        if state_path.exists() {
            Arc::new(Mutex::new(
                std::fs::read(state_path)
                    .map_err(Error::from)
                    .and_then(|state| serde_json::from_slice(&state).map_err(Into::into))
                    .unwrap_or_default(),
            ))
        } else {
            Default::default()
        }
    } else {
        Default::default()
    }
}

pub trait WindowExt {
    /// Restores this window state from disk
    fn restore_state(&self) -> tauri::Result<()>;
    fn set_state(&self) -> tauri::Result<()>;
}

impl<R: Runtime> WindowExt for WebviewWindow<R> {
    fn restore_state(&self) -> tauri::Result<()> {
        self.as_ref().window().restore_state()
    }
    fn set_state(&self) -> tauri::Result<()> {
        self.as_ref().window().set_state()
    }
}

impl<R: Runtime> WindowExt for Window<R> {
    fn restore_state(&self) -> tauri::Result<()> {
        let cache: tauri::State<'_, WindowStateCache> = self.state::<WindowStateCache>();
        let cache = cache.0.clone();
        let binding = cache.lock().unwrap();
        let state = binding.get(self.label());

        if state.is_none() {
            return Ok(());
        }

        let state = state.unwrap();

        if state.decorated {
            self.set_decorations(state.decorated)?;
        }

        if state.fullscreen {
            self.set_fullscreen(state.fullscreen)?;
        }
        self.set_size(LogicalSize {
            width: state.width,
            height: state.height,
        })?;

        if state.visible {
            self.show()?;
            self.set_focus()?;
        }

        if state.maximized {
            self.maximize()?;
        }

        let position: LogicalPosition<f64> = if state.maximized {
            (state.prev_x, state.prev_y).into()
        } else {
            (state.x, state.y).into()
        };
        self.set_position(position)?;

        Ok(())
    }
    fn set_state(&self) -> tauri::Result<()> {
        let cache = self.state::<WindowStateCache>();
        let cache = cache.0.clone();
        let label = self.label();

        let mut binding = cache.lock().unwrap();
        let state = binding.get_mut(label).unwrap();

        let pos = self.outer_position()?;
        let size = self.inner_size()?;

        state.x = pos.x as f64;
        state.y = pos.y as f64;
        state.width = size.width;
        state.height = size.height;
        state.decorated = self.is_decorated()?;
        state.fullscreen = self.is_fullscreen()?;
        state.maximized = self.is_maximized()?;
        state.visible = self.is_visible()?;

        Ok(())
    }
}

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.window")
        .setup(|app, _| {
            let cache = get_cache_config(app);
            app.manage(WindowStateCache(cache));
            Ok(())
        })
        .on_window_ready(|window| {
            let app = window.app_handle();
            let cache = app.state::<WindowStateCache>();
            let cache = cache.0.clone();
            let label = window.label();


            cache
                .lock()
                .unwrap()
                .entry(label.to_string())
                .or_insert_with(|| {
                    if let Ok(Some(current_monitor)) = window.current_monitor() {
                        let current_monitor_size = current_monitor.size();
                        let height = current_monitor_size.height;
                        let width = current_monitor_size.width;

                        return WindowState {
                            width: width / 3,
                            height: 56,
                            x: (width / 3) as f64,
                            // x: (width / 3) as f64,
                            y: (height / 4) as f64,
                            prev_x: 0,
                            prev_y: 0,
                            visible: true,
                            maximized: false,
                            decorated: false,
                            fullscreen: false,
                        };
                    }
                    Default::default()
                });
            _ = window.restore_state();
            let window_ = window.clone();
            window.on_window_event(move |e| match e {
                WindowEvent::CloseRequested { .. } => {
                    _ = window_.set_state();
                }
                _ => {}
            });
        })
        .on_event(|app, e| match e {
            RunEvent::ExitRequested { .. } => {
                _ = app.save_window_state();
            }

            _ => {}
        })
        .build()
}
