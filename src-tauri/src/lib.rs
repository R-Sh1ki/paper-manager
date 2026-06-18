mod commands;
mod db;
mod domain;
mod error;
mod services;

use commands::paper_cmd::{
    create_paper, delete_paper, get_paper, list_papers, update_paper,
};

use tauri::Manager;

#[tauri::command]
fn app_info() -> String {
    "Paper Manager backend is running.".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data directory!");

            let handle = app.handle().clone();
            
            tauri::async_runtime::block_on(async move {
                let pool = db::init_db(app_data_dir)
                    .await
                    .expect("failed to initialize database!");
                handle.manage(pool);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_info,
            create_paper,
            list_papers,
            get_paper,
            update_paper,
            delete_paper,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application!");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }



// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet, app_info])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
