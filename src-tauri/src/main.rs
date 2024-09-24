// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]




mod app;
mod window;


#[cfg(desktop)]
mod tray;





fn main() {
    app::run();
}
