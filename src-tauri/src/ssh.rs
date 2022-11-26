use std::io::Read;
use std::net::TcpStream;
use ssh2::Session;
//use std::error::Error;

#[derive(Default)]
pub struct Ssh {
    session : Option<Session>,
    host : String,
    user : String,
    password : String,
    private_key : String,
}

impl Ssh {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn connect(&mut self, host: &str, port: i16, user: &str, password: &str) 
        -> Result<(), String> {
        let tcp = match TcpStream::connect(format!("{}:{}", host, port)) {
            Err(e) => return Err(e.to_string()),
            Ok(o) => o,
        };

        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        if let Err(e) = session.handshake() {
            return Err(e.to_string());
        }
        if let Err(e) = session.userauth_password(user, password) {
            return Err(e.to_string());
        }
        //let pkey = std::path::Path::new("c:\\users\\san\\.ssh\\id_rsa");
        //session.userauth_pubkey_file("support", None, pkey, None);
        assert!(session.authenticated());
        self.session = Some(session);
        self.host = host.to_string();
        self.user = user.to_string();
        self.password = password.to_string();
        Ok(())
    }
    pub fn connect_with_key(&mut self, host: &str, port: i16, user: &str, pkey: &str) 
        -> Result<(), String> {
        let tcp = match TcpStream::connect(format!("{}:{}", host, port)) {
            Err(e) => return Err(e.to_string()),
            Ok(o) => o,
        };

        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        if let Err(e) = session.handshake() {
            return Err(e.to_string());
        }
        let private_key = std::path::Path::new(pkey);
        if let Err(e) = session.userauth_pubkey_file(user, None, private_key, None) {
            return Err(e.to_string());
        }
        assert!(session.authenticated());
        self.session = Some(session);
        self.host = host.to_string();
        self.user = user.to_string();
        self.private_key = String::from(private_key.as_os_str().to_string_lossy());
        Ok(())
    }


    pub fn disconnect(&mut self) -> Result<(), String> {
        if let Err(e) = self.session.as_ref().unwrap().disconnect(None,"",None) {
            return Err(e.to_string());
        }
        Ok(())
    }
    pub fn run(&mut self, cmd: &str) -> Result<String, String> {
        println!("running CMD: {}", cmd);
        let mut channel = match self.session.as_ref().unwrap().channel_session() {
            Err(e) => return Err(format!("Error: {}", e)),
            Ok(o) => o,
        };
        channel.exec(cmd).unwrap();
        let mut s = String::new();
        channel.stderr().read_to_string(&mut s).unwrap();
        if !s.trim().is_empty() {
            return Err(format!("stderr: {}",s));
        };
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close().unwrap();
        Ok(s.trim().to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn whoami() {
        let mut ssh = Ssh::new();
        ssh.connect("localhost", 22, "support", "support").unwrap();
        let output = ssh.run("whoami").unwrap();
        print!("{}", output);
        assert_eq!("support", output.as_str());
    }
}