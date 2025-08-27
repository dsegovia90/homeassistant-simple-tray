use std::collections::HashMap;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::{
    build_tray_menu,
    home_assistant::{self, ApiStatusResponse, BooleanEntity, HomeAssistant},
};

#[tauri::command]
pub async fn load_settings(app: tauri::AppHandle) -> Result<HomeAssistant, String> {
    Ok(HomeAssistant::load_settings(&app)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn check_api_status(
    app: tauri::AppHandle,
    app_url: &str,
    token: &str,
) -> Result<ApiStatusResponse, ApiStatusResponse> {
    let mut home_assistant = home_assistant::HomeAssistant::load_settings(&app)
        .await
        .map_err(|_e| {
            ApiStatusResponse::offline("Unable to load HomeAssistant settings.".to_string())
        })?;

    home_assistant
        .update_settings(&app, app_url, token)
        .await
        .map_err(|_e| ApiStatusResponse::offline("Unable to update settings.".to_string()))?;

    Ok(home_assistant.check_status().await?)
}

#[tauri::command]
pub async fn get_switch_entities(app: tauri::AppHandle) -> Result<Vec<BooleanEntity>, String> {
    let home_assistant = HomeAssistant::load_settings(&app)
        .await
        .map_err(|e| e.to_string())?;

    let response = home_assistant
        .get_switch_entities()
        .await
        .map_err(|e| e.to_string())?;
    dbg!(&response);
    Ok(response)
}

#[tauri::command]
pub async fn save_entity_to_store(
    app: tauri::AppHandle,
    entity: BooleanEntity,
    save: bool,
) -> Result<(), String> {
    let store = app.store("entities.json").map_err(|e| e.to_string())?;

    // Get existing entities or create empty vec
    let mut entities: HashMap<String, BooleanEntity> = match store.get("entities") {
        Some(value) => {
            let entities: HashMap<String, BooleanEntity> =
                serde_json::from_value(value.clone()).unwrap_or_else(|_| HashMap::new());
            entities
        }
        None => HashMap::new(),
    };

    if save {
        entities.insert(entity.id.clone(), entity);
    } else {
        entities.remove(&entity.id);
    }

    store.set("entities", serde_json::to_value(&entities).unwrap());

    build_tray_menu(app.app_handle())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn load_entities_from_store(app: tauri::AppHandle) -> Result<Vec<BooleanEntity>, String> {
    let store = app.store("entities.json").map_err(|e| e.to_string())?;

    let entities: HashMap<String, BooleanEntity> = match store.get("entities") {
        Some(value) => {
            let entities: HashMap<String, BooleanEntity> =
                serde_json::from_value(value.clone()).unwrap_or_else(|_| HashMap::new());
            entities
        }
        None => HashMap::new(),
    };

    Ok(entities.into_values().collect())
}
