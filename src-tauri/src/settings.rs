use serde::{Deserialize, Serialize};
use confy;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub server: String,
    pub user: String,
    pub password: String,
    pub port: i16,
    pub private_key: String,
    pub home_dir : String,
}

impl Default for Settings {
    fn default() -> Self { 
        let home = String::from(dirs::home_dir().unwrap().to_string_lossy());
        let pkey = String::from(std::path::Path::new(home.as_str()).join(".ssh").join("id_rsa_pem").to_string_lossy());
        Self { 
            server: "localhost".into(),
            user: "support".into(),
            password: "".into(),
            port: 22,
            home_dir : home,
            private_key: pkey,
            
        } 
    }
}
 
pub fn read_settings() -> Result<Settings, String> {
    let settings: Settings = match confy::load("rustyssh", None) {
        Err(e) =>{
            println!("{:?}", e);
            return Ok(Settings{..Default::default()});
        },
        Ok(settings)=> settings,
    };
    println!("{:?}", settings);
    Ok(settings)
}

pub fn write_settings(settings: Settings) -> Result<(), String> {
    println!("{:?}", settings);
    let s = Settings { password: "".into(), ..settings};
    match confy::store("rustyssh",None, &s) {
        Err(e) => {
            Err(e.to_string())
        },
        Ok(_) => Ok(()),
    }
}