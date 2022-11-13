#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mastodon::Mastodon;
use mastodon::Status;
use tauri::State;
use url::Url;

#[tauri::command]
async fn fetch_public_timeline(masto: State<'_, Mastodon>) -> Result<Vec<Status>, String> {
    masto.public_timeline().await.map_err(|e| {
        e.to_string()
    })
}

fn main() {
    let masto = Mastodon::new(Url::parse("https://hachyderm.io/api/v1/").unwrap())
        .expect("Failed to instantiate Mastodon client");
    tauri::Builder::default()
        .manage(masto)
        .invoke_handler(tauri::generate_handler![fetch_public_timeline])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
