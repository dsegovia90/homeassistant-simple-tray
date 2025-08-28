mod api_settings;
mod home_assistant;
mod tray;

use crate::tray::build_tray_menu;
use tauri::async_runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            // Spawn the tray menu creation in a separate task to avoid blocking
            async_runtime::spawn(async move {
                if let Err(e) = build_tray_menu(&app_handle).await {
                    eprintln!("Failed to build tray menu: {:?}", e);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            api_settings::check_api_status,
            api_settings::load_settings,
            api_settings::get_switch_entities,
            api_settings::save_entity_to_store,
            api_settings::load_entities_from_store,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, code, .. } => {
                if code.is_none() {
                    api.prevent_exit();
                } else {
                    dbg!("exit code: {:?}", code);
                }
            }
            _ => {}
        });
}
