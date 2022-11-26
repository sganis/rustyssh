#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod settings;
mod ssh;

use settings::Settings;
use std::sync::Mutex;
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct App {
    ssh: Mutex<ssh::Ssh>,
    connected: Mutex<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    filetype: String,
    size: u64,
    owner: String,
    modified: String,
    path: String,
    parent: String,
}

#[tauri::command]
fn read_settings() -> Result<Settings, String> {
    settings::read_settings()
}

#[tauri::command]
fn write_settings(settings: Settings) -> Result<(), String> {
    settings::write_settings(settings)
}


#[tauri::command]
fn connect(settings: Settings, app: State<App>) -> Result<(), String> {
    let mut _ssh = ssh::Ssh::new();
    match _ssh.connect(
        settings.server.as_str(), 
        settings.port, 
        settings.user.as_str(), 
        settings.password.as_str()) {
        Err(e) => {
            Err(e)
        },
        Ok(_) => {
            write_settings(settings).expect("Cannot write settings");
            let mut ssh = app.ssh.lock().unwrap();
            *ssh = _ssh;
            *app.connected.lock().unwrap() = true;
            println!("Connected");
            let output = ssh.run("whoami").unwrap();
            println!("{}", output);
            Ok(())
        }
    }   
}

#[tauri::command]
fn disconnect(app: State<App>) -> Result<(), String> {
    let mut ssh = app.ssh.lock().unwrap();
    ssh.disconnect()   
}

#[tauri::command]
fn ssh_run(command: String, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    //println!("CMD: {}", command);
    ssh.run(command.as_str())
}

#[tauri::command]
fn get_files(path: String, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.run(format!("ls -l --time-style=full-iso {} |grep -v total", path).as_str()) {
        Err(e) => Err(e),
        Ok(o) => {
            let mut files: Vec<File> = Vec::new();
            let lines: Vec<String> = o.split("\n").map(|s| s.to_string()).collect();
            for line in lines {
                let items: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
                if items.len() < 9 {
                    continue;
                }
                let mut fullpath = format!("{}/{}",path, items[8]);
                if fullpath.starts_with("//") { 
                    fullpath = String::from(&fullpath[1..]);
                }
                let filetype = match &items[0][0..1] {
                    "-" => "file",
                    "d" => "dir",
                    "l" => "link",
                    "c" => "cdev",
                    "b" => "bdev",
                    "s" => "sock",
                    "p" => "pipe",
                    &_ => "unknown",
                }.to_string();
                
                let file = File {
                    name: items[8].clone(),
                    filetype: filetype,
                    size: items[4].parse::<u64>().unwrap(),
                    owner: items[2].clone(),
                    modified: items[5].clone(),
                    path: fullpath,
                    parent: path.clone(),
                };
                println!("{:?}", file);
                files.push(file);
            }            
            Ok(serde_json::to_string(&files).unwrap())
        },
    }
}

fn main() {
    tauri::Builder::default()
        .manage(App {..Default::default() })
        .invoke_handler(tauri::generate_handler![
            read_settings,
            write_settings,
            connect,
            disconnect,
            ssh_run,
            get_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
