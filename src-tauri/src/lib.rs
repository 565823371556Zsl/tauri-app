use serde_json::{json, Value};
use std::sync::Arc;
use sysinfo::System;
use tauri::{Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn auto_start_monitoring(app_handle: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut sys = System::new_all();

        // 获取窗口实例，假设你有一个名为 "main" 的窗口
        if let Some(_window) = app_handle.get_webview_window("main") {
            loop {
                // 刷新数据
                sys.refresh_all();

                let mut cpus = vec![];

                // 获取cpu信息
                for cpu in sys.cpus() {
                    let cpu_usage = cpu.cpu_usage();
                    let frequency = cpu.frequency();
                    let name = cpu.name();
                    let vendor_id = cpu.vendor_id();
                    let brand = cpu.brand();
                    cpus.push(json!({
                        "cpu_usage":cpu_usage,
                        "frequency":frequency,
                        "name":name,
                        "vendor_id":vendor_id,
                        "brand":brand
                    }));
                }

                // 获取内存信息
                let memroy = json!({
                    "total_memory":sys.total_memory(),
                    "used_memory":sys.used_memory()
                });

                app_handle
                    .emit(
                        "system_stats",
                        json!({
                            "cpus":cpus,
                            "memroy":memroy
                        }),
                    )
                    .expect("事件传输失败");

                // 控制发送频率
                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            auto_start_monitoring(app_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
