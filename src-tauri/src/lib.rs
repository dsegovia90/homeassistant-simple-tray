mod api_settings;
mod home_assistant;

use std::collections::HashMap;

use home_assistant_rest::{post::StateParams, Client};
use tauri::{
    async_runtime,
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry,
};
use tauri_plugin_store::StoreExt;

use crate::{api_settings::load_entities_from_store, home_assistant::HomeAssistant};

pub async fn build_tray_menu(app: &AppHandle) -> Result<(), tauri::Error> {
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(
        app,
        "quit",
        "Quit SimpleTray completely",
        true,
        None::<&str>,
    )?;

    let tray_entities = load_entities_from_store(app.clone())
        .await
        .map_err(|_err| tauri::Error::NoBasename)?
        .into_iter()
        .map(|entity| {
            MenuItem::with_id(app, entity.id, entity.friendly_name, true, None::<&str>).unwrap()
        })
        .collect::<Vec<_>>();

    let mut menu_items: Vec<&dyn tauri::menu::IsMenuItem<Wry>> = vec![&settings];
    menu_items.extend(
        tray_entities
            .iter()
            .map(|e| e as &dyn tauri::menu::IsMenuItem<Wry>),
    );
    menu_items.push(&quit_i);

    let menu = Menu::with_items(app, &menu_items)?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(Image::from_path("icons/tray-icon.png")?)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                // Check if settings window already exists
                if let Some(window) = app.get_webview_window("settings") {
                    // If it exists, bring it to focus
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    // Create a new settings window
                    let _ = WebviewWindowBuilder::new(
                        app,
                        "settings",
                        WebviewUrl::App("index.html".into()),
                    )
                    .title("HA SimpleTray - Settings")
                    .inner_size(465.0, 600.0)
                    .build();
                }
            }
            "quit" => {
                app.exit(0);
            }
            id => {
                async_runtime::block_on(async move {
                    let ha = HomeAssistant::load_settings(app).await.unwrap();
                    ha.toggle_switch_entity(id.to_string()).await.unwrap();
                });
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            async_runtime::block_on(async move {
                build_tray_menu(app_handle).await.unwrap();
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
