
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use confy;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct App {
    number: Mutex<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    server: String,
    user: String,
    password: String,
    remember_me: bool,
}

impl Default for Settings {
    fn default() -> Self { Self { 
        server: "localhost".into(),
        user: "support".into(),
        password: "support".into(),
        remember_me: true,
    } }
}

#[tauri::command]
fn read_settings() -> Result<Settings, String> {
    let settings: Settings = match confy::load("rustyssh", None) {
        Err(e) =>{
            println!("{:?}", e);
            return Err(e.to_string());
        },
        Ok(settings)=> settings,
    };
    println!("{:?}", settings);
    Ok(settings)

}

#[tauri::command]
fn write_settings(settings: Settings) -> Result<(), String> {
    println!("{:?}", settings);
    match confy::store("rustyssh",None, &settings) {
        Err(e) => {
            println!("{:?}", e);
            Err(e.to_string())
        },
        Ok(_) => Ok(()),
    }
}

#[tauri::command]
fn increment(n: i32, app: State<App>) ->i32 {
    let mut _n = app.number.lock().unwrap();
    *_n += n;
    *_n
}

fn main() {
    let mut number = 0;

    tauri::Builder::default()
        .manage(App {..Default::default() })
        .invoke_handler(tauri::generate_handler![
            read_settings,
            write_settings,
            increment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
