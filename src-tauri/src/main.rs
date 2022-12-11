#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod settings;
mod ssh;
mod command;

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
    link: String,
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
fn connect_with_password(settings: Settings, app: State<App>) -> Result<(), String> {
    let mut _ssh = ssh::Ssh::new();
    match _ssh.connect_with_password(
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
fn connect_with_key(settings: Settings, app: State<App>) -> Result<(), String> {
    let mut _ssh = ssh::Ssh::new();
    let mut pkey = String::new();
    
    if settings.private_key.is_empty() {
        pkey = String::from(ssh::Ssh::private_key_path().to_string_lossy());
    }

    match _ssh.connect_with_key(
        settings.server.as_str(), 
        settings.port, 
        settings.user.as_str(), 
        pkey.as_str()) {
        Err(e) => {
            println!("{e}");
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
fn setup_ssh(settings: Settings) -> Result<(), String> {
    let host = settings.server.as_str();
    let port = settings.port; 
    let user = settings.user.as_str();
    let password = settings.password.as_str();
    ssh::Ssh::setup_ssh(host, port, user, password)
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
    let cmd = format!("/bin/sh -c \"/bin/ls -l --time-style=full-iso {path}|grep -v total\"");
    match ssh.run(&cmd) {
        Err(e) => {
            println!("{e}");
            Err(e)
        },
        Ok(o) => {
            let mut files: Vec<File> = Vec::new();
            let lines: Vec<String> = o.split("\n").map(|s| s.to_string()).collect();
            for line in lines {
                let items: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
                if items.len() < 9 {
                    continue;
                }
                let filetype = match &items[0][0..1] {
                    "-" => "REG",
                    "d" => "DIR",
                    "l" => "LINK",
                    "c" => "CDEV",
                    "b" => "BDEV",
                    "s" => "SOCK",
                    "p" => "PIPE",
                    &_ => "UNKNOWN",
                }.to_string();
                let owner = items[2].clone();
                // /dev has differen ls -l output
                let mut size_index = 4;
                let mut modified_index = 5;
                let mut name_index = 8;
                let mut link_index = 10;

                if items[4].contains(",") {
                    size_index = 5;
                    modified_index = 6;
                    name_index = 9;
                    link_index = 12;
                }
                
                let size = items[size_index].parse::<u64>().unwrap();
                let modified = items[modified_index].clone();                
                let name = items[name_index].clone();
                let mut fullpath = format!("{}/{}",path, name);

                if fullpath.starts_with("//") { 
                    fullpath = String::from(&fullpath[1..]);
                }
                let path = fullpath;
                let parent = path.clone();
            
                let link = match filetype.as_str() {
                    "LINK" => items[link_index].clone(),
                    &_ => "".to_string(),
                };

                let file = File { name,filetype,size, owner, modified, path, parent, link };
                //println!("{:?}", file);
                files.push(file);
            }            
            let mut result: Vec<&File> = files.iter().filter(|f| f.filetype=="DIR").collect();
            let mut onlyfiles: Vec<&File> = files.iter().filter(|f| f.filetype!="DIR").collect();
            result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            onlyfiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            result.extend(onlyfiles);
            Ok(serde_json::to_string(&result).unwrap())
        },
    }
}

#[tauri::command]
fn get_file_types(paths: Vec<String>, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    let cmd = format!("file -L \"{}\"", paths.join("\" \""));
    match ssh.run(&cmd) {
        Err(e) => {
            println!("{e}");
            Err(e)
        },
        Ok(o) => {
            let mut files: Vec<(String, bool)> = Vec::new();
            let lines: Vec<String> = o.split("\n").map(|s| s.to_string()).collect();
            for line in lines {
                let items: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
                if items.len() < 2 {
                    continue;
                }
                let path = items[0].clone();
                //println!("{:?}", &items);
                let start = items[1].len()-9;
                //println!("{}", &items[1][start..]);
                let is_dir = if items[1][start..].contains("directory") { true } else { false };
                files.push((path, is_dir));
            }
            println!("{:?}", &files);
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
            connect_with_password,
            connect_with_key,
            disconnect,
            setup_ssh,
            ssh_run,
            get_files,
            get_file_types,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
