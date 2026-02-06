use reqwest::RequestBuilder;
use std::time::Instant;
mod models;
use models::{HttpResponse, RequestPayload};

#[tauri::command]
async fn request(payload: RequestPayload) -> Result<HttpResponse, String> {
    let start = Instant::now();
    let client = reqwest::Client::new();

    let mut request: RequestBuilder = match payload.method {
        models::HttpMethod::Get => client.get(&payload.url),
        models::HttpMethod::Put => client.put(&payload.url),
        models::HttpMethod::Post => client.post(&payload.url),
        models::HttpMethod::Patch => client.patch(&payload.url),
        models::HttpMethod::Head => client.head(&payload.url),
        models::HttpMethod::Options => client.request(reqwest::Method::OPTIONS, &payload.url),
        models::HttpMethod::Delete => client.delete(&payload.url),
    };

    for header in payload.headers.iter().filter(|h| h.enabled) {
        request = request.header(&header.key, &header.value);
    }

    if payload.body.is_some() {
        request = request.body(payload.body.unwrap());
    }

    let response = request.send().await.map_err(|e| e.to_string())?;

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("")
        .to_string();
    let response_headers = response
        .headers()
        .iter()
        .map(|(name, value)| models::KeyValuePair {
            key: name.to_string(),
            value: value.to_str().unwrap_or("").to_string(),
            enabled: true,
        })
        .collect();
    let body = response.text().await.map_err(|e| e.to_string())?;
    let time_ms = start.elapsed().as_millis() as u64;

    Ok(HttpResponse {
        status,
        status_text,
        headers: response_headers,
        body,
        time_ms,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
