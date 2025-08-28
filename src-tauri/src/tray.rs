use crate::{api_settings::load_entities_from_store, home_assistant::HomeAssistant};
use tauri::{
    async_runtime,
    image::Image,
    menu::{Menu, MenuEvent, MenuItem},
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

    Ok(Menu::with_items(app, &menu_items)?)
}

pub async fn build_tray_menu(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = build_menu(app).await?;
    println!("Built menu!");

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
    println!("Built menu!");

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}
