use std::{collections::HashMap, sync::{Arc, Mutex}};

use tauri::{
    plugin::{Builder, TauriPlugin}, AppHandle, Manager, Wry
};

pub type EventCallbackFn = Box<dyn Fn(&str) + Send + Sync + 'static>;
pub type EventCallbackMulitFn = Box<dyn Fn(Vec<&str>) + Send + Sync + 'static>;

pub struct GlobalEvent {
    event: Arc<Mutex<HashMap<String, Vec<EventCallbackFn>>>>,
    event_mulit: Arc<Mutex<HashMap<String, Vec<EventCallbackMulitFn>>>>,
}

impl GlobalEvent {
    pub fn on(&self, event: &str, callback: EventCallbackFn) {
        let _event = event.to_string();
        let mut binding = self.event.lock().unwrap();
        let callback_list = binding.entry(_event).or_insert_with(Default::default);
        callback_list.push(callback);
    }

    pub fn send(&self, event: &str, msg: &str) {
        if let Some(callback_list) = self.event.lock().unwrap().get(event) {
            callback_list.iter().for_each(|callback| callback(msg));
        }
    }

    pub fn on_mulit(&self, event: &str, callback: EventCallbackMulitFn) {
        let _event = event.to_string();
        let mut binding = self.event_mulit.lock().unwrap();
        let callback_list = binding.entry(_event).or_insert_with(Default::default);
        callback_list.push(callback);
    }
    pub fn send_mulit(&self, event: &str, msg: Vec<&str>) {
        if let Some(callback_list) = self.event_mulit.lock().unwrap().get(event) {
            callback_list.iter().for_each(|callback| callback(msg.clone()));
        }
    }
}

impl Default for GlobalEvent {
    fn default() -> Self {
        Self {
            event: Default::default(),
            event_mulit: Default::default(),
        }
    }
}

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("core.event")
        .setup(|app, _| {
            let state: GlobalEvent = Default::default();
            app.manage(state);
            Ok(())
        })
        .on_event(|_, e| match e {
            _ => {}
        })
        .build()
}









pub trait GlobalEventAppHandleExt {
    fn global_event(&self) -> &GlobalEvent;
}

impl GlobalEventAppHandleExt for AppHandle {
    fn global_event(&self) -> &GlobalEvent {
        self.state::<GlobalEvent>().inner()
    }
}
