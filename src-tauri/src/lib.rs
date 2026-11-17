pub mod commands;
pub mod ledger;
pub mod models;
pub mod organizer;
pub mod scanner;
pub mod sorter;

use commands::{
    check_undo_available, get_default_downloads_dir, get_image_data_url, organize_selected,
    scan_folder, undo_last,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_default_downloads_dir,
            scan_folder,
            organize_selected,
            check_undo_available,
            undo_last,
            get_image_data_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
