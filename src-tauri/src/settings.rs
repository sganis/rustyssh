use serde::{Deserialize, Serialize};
use confy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub server: String,
    pub user: String,
    pub password: String,
    pub port: i16,
    pub remember_me: bool,
}

impl Default for Settings {
    fn default() -> Self { 
        Self { 
            server: "localhost".into(),
            user: "support".into(),
            password: "support".into(),
            port: 22,
            remember_me: true,
        } 
    }
}

pub fn read_settings() -> Result<Settings, String> {
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

pub fn write_settings(settings: Settings) -> Result<(), String> {
    println!("{:?}", settings);
    match confy::store("rustyssh",None, &settings) {
        Err(e) => {
            println!("{:?}", e);
            Err(e.to_string())
        },
        Ok(_) => Ok(()),
    }
}