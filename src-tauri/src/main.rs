#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod settings;
mod ssh;
mod command;

use settings::Settings;
use std::{sync::Mutex, fs::OpenOptions, collections::HashMap};
use tauri::State;
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    link_path: String,
    is_dir: bool,
    is_link: bool,
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
    ssh.run(&command)
}

#[tauri::command]
fn get_files(path: String, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    let mut files: Vec<File> = Vec::new();
    let cmd = format!("/bin/sh -c \"/bin/ls -lH --time-style=full-iso {path}|grep -v total\"");
    let o = ssh.run(&cmd)?;
    let lines: Vec<String> = o.split("\n").map(|s| s.to_string()).collect();

    for line in lines {
        let items: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if items.len() < 9 {
            continue;
        }
        let mut is_dir = false;
        let mut is_link = false;
        let filetype = match &items[0][0..1] {
            "-" => "REG",
            "d" => {is_dir = true; "DIR"},
            "l" => {is_link = true; "LINK"},
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
    
        let link_path = match filetype.as_str() {
            "LINK" => items[link_index].clone(),
            &_ => "".to_string(),
        };

        let file = File { name, filetype, size, owner, modified, path, 
            parent, link_path, is_dir, is_link };
        //println!("{:?}", file);
        files.push(file);
    }       

    let symlinks: Vec<String> = files.iter()
        .filter(|f| f.is_link)
        .map(|f| f.path.clone()).collect();
    //println!("symlinks: {:?}", symlinks);
    let cmd = format!("file -L \"{}\"", symlinks.join("\" \""));
    let o = ssh.run(&cmd)?;
    let lines: Vec<String> = o.split("\n").map(|s| s.to_string()).collect();
    
    for line in lines {
        let items: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
        if items.len() < 2 {
            continue;
        }
        let path = items[0].clone();
        let start = items[1].len()-9;
        let is_dir = if items[1][start..].contains("directory") { true } else { false };
        //files.push((path, is_dir));
        for f in  &mut files {
            if f.path == path {
                f.is_dir = is_dir;
                break;
            }
        }
    }
    // println!("{:?}", &files);
    let mut result: Vec<&File> = files.iter().filter(|f| f.is_dir).collect();
    let mut onlyfiles: Vec<&File> = files.iter().filter(|f| !f.is_dir).collect();
    result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    onlyfiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    result.extend(onlyfiles);

    Ok(serde_json::to_string(&result).unwrap())
}

#[tauri::command]
fn get_page(path: String, page: i32, records_per_page: i32, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    let o = ssh.run(&format!("file {path}"))?;
    let is_textfile = o.contains(" text");
    if is_textfile {
        let prev = (page-1)*records_per_page;
        let first = prev + 1;
        let last = prev + records_per_page;
        let cmd = format!("sed -n -e \"{first},{last}p\" -e \"{last}q\" {path}");
        let o = ssh.run(&cmd)?;

        Ok(serde_json::to_string(&o).unwrap())
    } else {
        Ok(serde_json::to_string("Binary file").unwrap())
    }
}

#[tauri::command]
fn download(path: String, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.download(&path) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("file saved to: {path}");
            Ok(o)
        },
    }
}
#[tauri::command]
fn upload(path: String, app: State<App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    // if let Err(e) = match ssh.download(&path) {
    //     return Err(format!("Failed to download file {}: {}", path, e));
    // }

    // let (mut remote_file, stat) = ssh.scp_recv(Path::new("/home/support/file.txt")).unwrap();
    // println!("remote file size: {}", stat.size());
    // let mut content = Vec::new();
    // remote_file.read_to_end(&mut content).unwrap();
    // let path = "file.txt";
    // let file = OpenOptions::new().write(true).open("file.txt")?;
    // file.write_all(&content);
    // println!("file saved to: {path}");
    Ok(String::new())
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
            get_page,
            download,
            upload,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
