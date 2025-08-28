use derive_builder::Builder;
use home_assistant_rest::get::{StateEntry, StateEnum};
use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Deserialize, Serialize, Builder)]
#[builder(derive(Deserialize))]
pub struct HomeAssistant {
    app_url: String,
    token: String,
    #[serde(skip)]
    #[builder_field_attr(serde(skip))]
    reqwest_client: reqwest::Client,
}

#[derive(Debug, Serialize)]
pub enum ApiStatus {
    Online,
    Offline,
}

#[derive(Debug, Serialize)]
pub struct ApiStatusResponse {
    status: ApiStatus,
    message: String,
}

impl ApiStatusResponse {
    pub fn offline(message: String) -> Self {
        Self {
            status: ApiStatus::Offline,
            message,
        }
    }

    pub fn online(message: String) -> Self {
        Self {
            status: ApiStatus::Online,
            message,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BooleanEntity {
    pub id: String,
    pub state: String,
    pub friendly_name: String,
}

impl TryFrom<StateEntry> for BooleanEntity {
    type Error = String;

    fn try_from(entry: StateEntry) -> Result<Self, Self::Error> {
        let state = entry
            .state
            .unwrap_or(StateEnum::String("unavailable".to_string()));

        let state = match state {
            StateEnum::String(s) => Ok(s),
            _ => Err("Unable to convert state to string".to_string()),
        }?;

        let friendly_name = entry
            .attributes
            .get("friendly_name")
            .unwrap_or_default()
            .as_str()
            .map_or("".to_string(), |s| s.to_string());

        Ok(BooleanEntity {
            id: entry.entity_id,
            state,
            friendly_name,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ToggleSwitchResponse {
    pub entity_id: String,
    pub state: String,
    pub attributes: ToggleSwitchAttributes,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ToggleSwitchAttributes {
    pub friendly_name: String,
}

impl HomeAssistant {
    pub async fn load_settings(app: &tauri::AppHandle) -> Result<Self, anyhow::Error> {
        let store = app.store("settings.json")?;
        let client = reqwest::Client::new();

        match store.get("settings") {
            Some(value) => {
                let mut settings: HomeAssistantBuilder = serde_json::from_value(value.clone())?;
                settings.reqwest_client = Some(client);
                Ok(settings.build()?)
            }
            None => Ok(HomeAssistant {
                app_url: "".to_string(),
                token: "".to_string(),
                reqwest_client: client,
            }),
        }
    }

    pub async fn update_settings(
        &mut self,
        app: &tauri::AppHandle,
        new_app_url: &str,
        new_token: &str,
    ) -> Result<(), anyhow::Error> {
        self.app_url = new_app_url.to_string();
        self.token = new_token.to_string();

        let store = app.store("settings.json")?;

        store.set("settings", serde_json::to_value(&self)?);
        store.save()?;
        Ok(())
    }

    pub async fn check_status(&self) -> Result<ApiStatusResponse, ApiStatusResponse> {
        let request = self
            .reqwest_client
            .get(format!("{}/api/", self.app_url))
            .bearer_auth(&self.token)
            .build()
            .map_err(|_e| ApiStatusResponse {
                status: ApiStatus::Offline,
                message: "Unable to build request".to_string(),
            })?;

        #[derive(Debug, Deserialize)]
        struct HomeAssistantResponse {
            message: String,
        }

        let response = self
            .reqwest_client
            .execute(request)
            .await
            .map_err(|_e| ApiStatusResponse {
                status: ApiStatus::Offline,
                message: "Unable to connect to Home Assistant".to_string(),
            })?
            .json::<HomeAssistantResponse>()
            .await
            .map_err(|_e| ApiStatusResponse {
                status: ApiStatus::Offline,
                message: "Unable to parse response".to_string(),
            })?;

        Ok(ApiStatusResponse::online(response.message))
    }

    pub async fn get_switch_entities(&self) -> Result<Vec<BooleanEntity>, anyhow::Error> {
        let request = self
            .reqwest_client
            .get(format!("{}/api/states", self.app_url))
            .bearer_auth(&self.token)
            .build()?;

        let response = self
            .reqwest_client
            .execute(request)
            .await?
            .json::<Vec<StateEntry>>()
            .await?;

        Ok(response
            .into_iter()
            .filter_map(|entity| {
                if !entity.entity_id.starts_with("switch.") {
                    return None;
                }

                if entity
                    .state
                    .eq(&Some(StateEnum::String("unavailable".to_string())))
                {
                    return None;
                }

                match entity.try_into() {
                    Ok(entity) => Some(entity),
                    Err(_e) => None,
                }
            })
            .collect())
    }

    pub async fn toggle_switch_entity(
        &self,
        id: String,
    ) -> Result<Vec<ToggleSwitchResponse>, anyhow::Error> {
        #[derive(Serialize)]
        struct ToggleSwitchPayload {
            entity_id: String,
        }

        let json = ToggleSwitchPayload { entity_id: id };

        let request = self
            .reqwest_client
            .post(format!("{}/api/services/switch/toggle", self.app_url))
            .bearer_auth(&self.token)
            .json(&json)
            .build()?;

        let response = self
            .reqwest_client
            .execute(request)
            .await?
            .json::<Vec<ToggleSwitchResponse>>()
            .await?;

        Ok(response)
    }
}
