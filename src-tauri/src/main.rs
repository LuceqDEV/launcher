// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

mod macros;
mod helper;
mod commands;

pub const VERSION_URL: &'static str = "https://raw.githubusercontent.com/HabboOriginsLauncher/update/main/version.txt";
pub const CLIENURLS_URL: &'static str = "https://origins.habbo.com/gamedata/clienturls";
pub const LOOKUP_URL: &'static str = "https://origins.habbo.com/api/public/users?name=";

pub(crate) fn get_clienturls() -> &'static Option<ClientUrls> {
    static VAL: OnceLock<Option<ClientUrls>> = OnceLock::new();
    VAL.get_or_init(|| None)
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct ClientUrls {
    #[serde(rename(deserialize = "shockwave-windows-version"))]
    shockwave_windows_version: String,
    #[serde(rename(deserialize = "shockwave-windows"))]
    shockwave_windows: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            use tauri_plugin_notification::NotificationExt;

            let client = reqwest::blocking::Client::new();
            let res = client
                .get(crate::VERSION_URL)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .header(reqwest::header::USER_AGENT, "Mozilla/1.22 (compatible; MSIE 5.01; PalmOS 3.0) EudoraWeb 2")
                .send()
                .unwrap();

            let content = res.text().unwrap();
            let content_len = content.len();
            if content_len > 0 {
                if content[0..(content_len - 1)] != *app.config().version.as_ref().unwrap() {
                    app.notification()
                    .builder()
                    .title("Habbo:Origins Launcher update!")
                    .body("A new version is available! Get the latest version here: https://github.com/HabboOriginsLauncher/update/releases")
                    .show()
                    .unwrap();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::installed_version, commands::clienturls, commands::check_update, commands::install_update, commands::launch, commands::user_lookup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
