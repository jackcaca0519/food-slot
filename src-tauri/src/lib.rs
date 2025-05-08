// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use reqwest;
use tauri::Manager;

#[derive(Debug, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize)]
struct Restaurant {
    name: String,
    rating: Option<f64>,
    address: String,
    place_id: String,
}

#[tauri::command]
async fn get_restaurants(location: Location) -> Result<Vec<Restaurant>, String> {
    let api_key = std::env::var("GOOGLE_API_KEY").map_err(|_| "Missing API key")?;

    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=2000&type=restaurant&key={}",
        location.lat, location.lng, api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request error: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;

    if let Some(results) = response.get("results").and_then(|v| v.as_array()) {
        let restaurants: Vec<Restaurant> = results
            .iter()
            .filter_map(|item| {
                Some(Restaurant {
                    name: item.get("name")?.as_str()?.to_string(),
                    rating: item.get("rating").and_then(|r| r.as_f64()),
                    address: item.get("vicinity")?.as_str()?.to_string(),
                    place_id: item.get("place_id")?.as_str()?.to_string(),
                })
            })
            .collect();
        Ok(restaurants)
    } else {
        Err("No results found".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_restaurants])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                if let Some(main_window) = app.get_webview_window("main") {
                    main_window.open_devtools();
                    main_window.close_devtools(); // 可留可不留
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
