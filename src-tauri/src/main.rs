#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod settings;
mod ssh;
mod command;
mod util;

//use serde_json::json;
use tauri::Window;
use settings::Settings;
use std::sync::Mutex;
use tauri::State;
use serde::{Deserialize, Serialize};
use chrono::prelude::{DateTime, NaiveDateTime, Utc};

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
fn connect_with_password(settings: Settings, app: State<'_,App>) -> Result<(), String> {
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
fn connect_with_key(settings: Settings, app: State<'_,App>) -> Result<(), String> {
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
async fn setup_ssh(settings: Settings) -> Result<(), String> {
    let host = settings.server.as_str();
    let port = settings.port; 
    let user = settings.user.as_str();
    let password = settings.password.as_str();
    ssh::Ssh::setup_ssh(host, port, user, password)
}

#[tauri::command]
async fn ssh_run(command: String, app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    ssh.run(&command)
}

#[tauri::command]
async fn get_new_filename(path: String) -> Result<String, String> {
    Ok(util::new_filename(&path))
}


#[tauri::command]
async fn get_files(path: String, hidden: bool, app: State<'_,App>) 
-> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    let mut files: Vec<File> = Vec::new();

    let sftpfiles = match ssh.sftp_readdir(&path) {
        Err(e) => return Err(e),
        Ok(o) => o,
    };

    //println!("{:?}",sftpfiles);

    for (p, stat) in sftpfiles {
        let path = String::from(p.to_string_lossy()).replace("\\","/");
        let name = String::from(p.file_name().unwrap().to_string_lossy());
        
        if !hidden && name.starts_with(".") {
            continue
        }

        let file_type = stat.file_type();        
        let mut is_dir = false;
        let mut is_link = false;
        let filetype = match file_type {
            ssh2::FileType::RegularFile => "REG",
            ssh2::FileType::Directory => {is_dir = true; "DIR"},
            ssh2::FileType::Symlink => {is_link = true; "LINK"},
            ssh2::FileType::CharDevice => "CDEV",
            ssh2::FileType::BlockDevice => "BDEV",
            ssh2::FileType::Socket => "SOCK",
            ssh2::FileType::NamedPipe => "PIPE",
            _ => "UNKNOWN",
        }.to_string();
        let size = stat.size.unwrap();
        let naive_time = NaiveDateTime::from_timestamp_opt(stat.mtime.unwrap() as i64, 0).unwrap();
        let datetime: DateTime<Utc> = DateTime::from_utc(naive_time, Utc);
        let modified = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let owner = stat.uid.unwrap().to_string();

        // if name.starts_with("/") { 
        //     name = String::from(&name[1..]);
        // }
        // let mut fullpath = format!("{}/{}",path, name);

        // if fullpath.starts_with("//") { 
        //     fullpath = String::from(&fullpath[1..]);
        // }
        // let path = fullpath;
        let parent = path.clone();
    
        // let link_path = match filetype.as_str() {
        //     "LINK" => items[link_index].clone(),
        //     &_ => "".to_string(),
        // };
        let mut link_path = "".to_string();

        // resolve link
        if file_type.is_symlink() {
            link_path = match ssh.sftp_realpath(&path)  {
                Err(e) => {
                    println!("error {path}: {e}");
                    "n/a".to_string()
                },
                Ok(o) => {
                    println!("realpath {path} -> {}", o.0);   
                    is_dir = o.1.is_dir();
                    o.0
                },
            };


        }

        let file = File { 
            name, filetype, size, owner, modified, path, 
            parent, link_path, is_dir, is_link 
        };
        
        //println!("{:?}", file);
        files.push(file);
    }     
    let mut result: Vec<&File> = files.iter().filter(|f| f.is_dir).collect();
    let mut onlyfiles: Vec<&File> = files.iter().filter(|f| !f.is_dir).collect();
    result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    onlyfiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    result.extend(onlyfiles);
    //println!("{}", result.len());  
    //println!("{:?}", result);
    Ok(serde_json::to_string(&result).unwrap())
}


#[tauri::command]
async fn get_page(path: String, page: i32, records_per_page: i32, app: State<'_,App>) -> Result<String, String> {
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
async fn save_file(path: String, data: String, app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    println!("data: {data}");
    ssh.sftp_save(&path, &data).expect("Cannot save file");
    Ok(serde_json::to_string("true").unwrap())
    
}

#[tauri::command]
async fn download(
    remotepath: String, 
    localpath: String,
    window: Window, 
    app: State<'_, App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.scp_download(&remotepath, &localpath, window) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("file saved to: {localpath}");
            Ok(serde_json::to_string(&o).unwrap())
        },
    }
}


#[tauri::command]
async fn upload(
    localpath: String,
    remotepath: String, 
    window: Window,
    app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.scp_upload(&localpath, &remotepath, window) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("file uploaded to: {remotepath}");
            Ok(serde_json::to_string(&o).unwrap())
        },
    }
}
#[tauri::command]
async fn mkdir(remotepath: String, app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.sftp_mkdir(&remotepath) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("new folder created: {remotepath}");
            Ok(serde_json::to_string(&o).unwrap())
        },
    }
}

#[tauri::command]
async fn rmdir(remotepath: String, app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.sftp_rmdir(&remotepath) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("folder deleted: {remotepath}");
            Ok(serde_json::to_string(&o).unwrap())
        },
    }
}
#[tauri::command]
async fn delete(remotepath: String, app: State<'_,App>) -> Result<String, String> {
    let mut ssh = app.ssh.lock().unwrap();
    match ssh.sftp_delete(&remotepath) {
        Err(e) => Err(e),
        Ok(o) => {
            println!("Deleted: {remotepath}");
            Ok(serde_json::to_string(&o).unwrap())
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
            get_new_filename,
            get_page,
            save_file,
            download,
            upload,
            mkdir,
            rmdir,
            delete,
            
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
