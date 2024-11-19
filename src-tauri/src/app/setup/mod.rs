use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::plugin::AppHandleExt;
use anyhow::Result;
use tauri::{App, Manager};

use super::plugin::{
    event::GlobalEvent,
    shortcut::{self, registry},
};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    // 接收错误 保存日志
    if let Err(err) = setup(app) {
        log::error!("{}", err);
    }
    Ok(())
}

fn setup(app: &App) -> Result<(), Box<dyn Error>> {
    let app_handle = app.app_handle();

    // 注册快捷键
    shortcut::setup(app_handle);
    registry(
        app_handle,
        "Alt+Space",
        "core:hotkey:setup:open.main.window",
    )?;

    // 加载 第三方插件
    app_handle.load_plugins();

    // 添加 plugin 事件
    let global_event = app.state::<GlobalEvent>().inner();
    // let _app = app.app_handle().clone();
    let _app = Arc::new(Mutex::new(app.app_handle().clone()));
    let __app = _app.clone();
    global_event.on_mulit(
        "registry_hotkey",
        Box::new(move |msg| {
            let app = &*__app.lock().unwrap();
            if let Err(err) = registry(app, msg[0], msg[1]) {
                log::error!("{}", err)
            }
        }),
    );



    let __app = _app.clone();
    global_event.on("plugin:event", Box::new(move |msg| {
        let app = &*__app.lock().unwrap();
    }));

    Ok(())
}
