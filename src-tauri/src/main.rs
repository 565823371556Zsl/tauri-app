// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri_app_lib::run()
}

// use sysinfo::{System, SystemExt};
// use tauri::{Manager, State};
// use std::sync::{
//     atomic::{AtomicBool, Ordering},
//     Arc,
// };
// use serde_json::json;
// use chrono::Local;

// // 共享监控状态
// struct MonitorState {
//     is_running: AtomicBool,
//     interval: u64,
// }

// // 自动启动的核心实现
// fn auto_start_monitoring(app: &tauri::AppHandle, state: &Arc<MonitorState>) {
//     let app_clone = app.clone();
//     let state_clone = Arc::clone(state);

//     std::thread::spawn(move || {
//         let mut sys = System::new();
//         while state_clone.is_running.load(Ordering::Relaxed) {
//             sys.refresh_all();

//             let payload = json!({
//                 "cpu": sys.global_cpu_info().cpu_usage(),
//                 "memory": sys.used_memory(),
//                 "timestamp": Local::now().timestamp_millis()
//             });

//             if let Err(e) = app_clone.emit_all("system_stats", payload) {
//                 eprintln!("[ERROR] 事件推送失败: {}", e);
//                 break;
//             }

//             std::thread::sleep(std::time::Duration::from_secs(state_clone.interval));
//         }
//         state_clone.is_running.store(false, Ordering::SeqCst);
//     });
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_os::init())
//         .plugin(tauri_plugin_opener::init())
//         .setup(|app| {
//             // 初始化监控状态
//             let monitor_state = Arc::new(MonitorState {
//                 is_running: AtomicBool::new(true), // 默认自动启动
//                 interval: 1,
//             });

//             // 将状态托管到应用
//             app.manage(monitor_state.clone());

//             // 自动启动监控线程
//             auto_start_monitoring(&app.handle(), &monitor_state);

//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![
//             // 保留停止命令用于主动控制
//             stop_monitoring
//         ])
//         .build()
//         .expect("error while building tauri application")
//         .run(|app, event| {
//             if let tauri::RunEvent::Exit = event {
//                 // 应用退出时自动停止
//                 let state: &Arc<MonitorState> = app.state();
//                 state.is_running.store(false, Ordering::SeqCst);
//             }
//         });
// }

// #[tauri::command]
// fn stop_monitoring(state: State<'_, Arc<MonitorState>>) {
//     state.is_running.store(false, Ordering::SeqCst);
// }
