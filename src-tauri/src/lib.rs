use serde_json::json;
use sysinfo::System;
use tauri::{Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 自动启动的核心实现
fn auto_start_monitoring(app: &tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut sys = System::new_all();
        // 获取窗口实例，假设你有一个名为 "main" 的窗口
        if let Some(window) = app.get_webview_window("main") {
            loop {
                // 在后台线程中执行某些操作，然后向前端发送事件
                sys.refresh_cpu_usage(); // Refreshing CPU usage.
                for cpu in sys.cpus() {
                    window
                        // .emit("system_stats", {
                        //     "data":"cpu数据",
                        // })
                        .expect("Failed to emit event");
                    print!("{}% ", cpu.cpu_usage());
                }

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

                // std::thread::sleep(std::time::Duration::from_secs(1)); // 控制发送频率
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| Ok(()))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
