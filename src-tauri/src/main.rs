// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tower::common::DEV_MODE;
use tracing::Level;
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(if DEV_MODE { Level::DEBUG } else { Level::INFO })
        .init();

    tauri_lib::run()
}
