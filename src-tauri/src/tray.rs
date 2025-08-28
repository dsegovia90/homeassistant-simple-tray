use crate::{api_settings::load_entities_from_store, home_assistant::HomeAssistant};
use tauri::{
    async_runtime,
    image::Image,
    menu::{Menu, MenuBuilder, MenuEvent},
    tray::TrayIconBuilder,
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry,
};

fn menu_event_handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
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
    }
}

async fn build_menu(app: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    load_entities_from_store(app.clone())
        .await
        .map_err(|_err| tauri::Error::NoBasename)?
        .into_iter()
        .fold(MenuBuilder::new(app), |builder, entity| {
            builder.text(entity.id, entity.friendly_name)
        })
        .separator()
        .text("settings", "Settings")
        .text("quit", "Quit Completely")
        .build()
}

pub async fn build_tray_menu(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = build_menu(app).await?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
        tray.on_menu_event(menu_event_handler);
    } else {
        let _tray = TrayIconBuilder::with_id("main")
            .icon(Image::from_path("icons/tray-icon.png")?)
            .menu(&menu)
            .on_menu_event(menu_event_handler)
            .build(app)?;
        app.remove_tray_by_id("startup");
    }

    Ok(())
}

pub async fn rebuild_tray_menu(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = build_menu(app).await?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}
