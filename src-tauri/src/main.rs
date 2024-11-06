// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod plugin;
mod app;
mod library;
mod util;



fn main() {
    app::Applican::run();
}
