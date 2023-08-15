// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use std::fs;
use tauri_plugin_log::LogTarget;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::LogDir,
                    LogTarget::Stdout,
                    LogTarget::Webview,
                    LogTarget::Folder("logs".into()),
                ])
                .level(LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![save_file, app_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(name: &str, bytes: &[u8]) -> i32 {
    let path = format!("./books/{}", name);

    // 如果出错了返回 401，判断fs::write是否出错
    if let Err(_) = fs::write(path, bytes) {
        return 401;
    } else {
        return 200;
    }
}
#[tauri::command]
fn app_dir() -> String {
    // 获取books文件夹的绝对路径
    let path = format!("{}/books", std::env::current_dir().unwrap().display());
    return path;
}
