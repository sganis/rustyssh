
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use ssh2::Session;
use std::path::PathBuf;
use super::command;

#[derive(Default)]
pub struct Ssh {
    session : Option<Session>,
    host : String,
    user : String,
    password : String,
    private_key : String,
}

#[allow(dead_code)]
impl Ssh {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn supported_algs() -> String {
        let ssh = Session::new().unwrap();
        println!("HostKey: {:?}", ssh.supported_algs(ssh2::MethodType::HostKey).unwrap());
        println!("CryptCs: {:?}", ssh.supported_algs(ssh2::MethodType::CryptCs).unwrap());
        println!("Kex: {:?}", ssh.supported_algs(ssh2::MethodType::Kex).unwrap());
        println!("MacCs: {:?}", ssh.supported_algs(ssh2::MethodType::MacCs).unwrap());
        println!("CompCs: {:?}", ssh.supported_algs(ssh2::MethodType::CompCs).unwrap());

        "supported flags above".to_string()
    }
    pub fn private_key_path() -> PathBuf {
        let home = dirs::home_dir().unwrap();
        let prikey = home.join(".ssh").join("id_rsa");
        PathBuf::from(&prikey)
        
    }
    pub fn public_key_path() -> PathBuf {
        let home = dirs::home_dir().unwrap();
        let pubkey = home.join(".ssh").join("id_rsa.pub").clone();
        PathBuf::from(&pubkey)
    }    
    pub fn has_private_key() -> bool {     
        Ssh::private_key_path().exists()
        
    }
    pub fn has_public_key() -> bool {      
        Ssh::public_key_path().exists()
    }
    fn generate_public_key() -> Result<(), String> {
        let seckey = Ssh::private_key_path();
        let pubkey = Ssh::public_key_path();
        
        let cmd = format!("ssh-keygen -f {} -y > {}", seckey.display(), pubkey.display());
        let (_,e,_) = command::run(&cmd);
        
        if e.len()>0 {
            Err(e)
        } else {
            Ok(())
        }
    }
    fn generate_keys() -> Result<(), String> {
        let seckey = Ssh::private_key_path();
        
        let cmd = format!("ssh-keygen -m PEM -N \"\" -f {}", seckey.display());
        let (_,e,_) = command::run(&cmd);
        
        if e.len()>0 {
            Err(e)
        } else {
            Ok(())
        }
    }
    fn transfer_public_key(host: &str, port: i16, user: &str, password: &str) -> Result<(), String> {
        let pubkeytext = std::fs::read_to_string(&Ssh::public_key_path()).unwrap().trim().to_string();
        let cmd = format!("exec sh -c \"cd; umask 077; mkdir -p .ssh; echo '{}' >> .ssh/authorized_keys\"",
                        pubkeytext);
        println!("{cmd}");
        let mut ssh = Ssh::new();
        if let Err(e) = ssh.connect_with_password(host, port, user, password) {
            println!("Error transfering keys, login with password: {e}");
            return Err(e);
        }
        if let Err(e) = ssh.run(&cmd) {
            println!("Error transfering keys, running command: {e}");            
            Err(e)
        } else {
            Ok(())
        }

    }
    fn test_ssh(host: &str, port: i16, user: &str) -> Result<(), String> {
        if !Ssh::has_private_key() {
            return Err("No private key".to_string());
        }
        let pkey = Ssh::private_key_path();
        let mut ssh = Ssh::new();
        if let Err(e) = ssh.connect_with_key(host, port, user, pkey.to_str().unwrap()) {
            Err(e)
        } else {        
            Ok(())
        }
    }
    pub fn setup_ssh(host: &str, port: i16, user: &str, password: &str) -> Result<(), String> {
        if !Ssh::has_private_key() {
            if let Err(e) = Ssh::generate_keys() {
                return Err(format!("Could not generate private key: {e}"));
            }
        }
        if !Ssh::has_public_key() {
            if let Err(e) = Ssh::generate_public_key() {
                return Err(format!("Could not generate public key: {e}"));
            }         
        }
        if Ssh::test_ssh(host, port, user).is_err() {
            if let Err(e) = Ssh::transfer_public_key(host, port, user, password) {
                return Err(format!("Could not transfer public key: {e}"));
            }
            if let Err(e) = Ssh::test_ssh(host, port, user) {
                return Err(format!("Test ssh failed: {e}"));
            }
        }
        Ok(())
    }
    
    fn _get_tcp(&mut self, host: &str, port: i16) -> Result<TcpStream, String> {
        let timeout = Duration::new(5, 0); // 5 secs
        let addresses: Vec<_> = match format!("{}:{}", host, port).to_socket_addrs() {
            Err(e) => {
                println!("Unable to resolve address: {}:{}  {:?}",host, port, e);
                return Err(e.to_string())
            },
            Ok(o) => o.collect(),
        };
        let mut tcp = None;
        let mut error = String::new();

        for addr in addresses {
            match TcpStream::connect_timeout(&addr, timeout) {
                Err(e) => {
                    error.push_str(&format!("tcp error: {:?}\n", e));
                    continue;
                },
                Ok(o) => {
                    println!("connected to: {:?}", addr);
                    tcp = Some(o);
                    break;  
                },
            };
        }

        if tcp.is_none() {
            return Err(error);
        }

        Ok(tcp.unwrap())
    }

    pub fn connect_with_password(&mut self, 
        host: &str, port: i16, user: &str, password: &str) -> Result<(), String> {
        
        let tcp = match self._get_tcp(host, port) {
            Err(e) => return Err(e),
            Ok(o) => o,
        };

        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);

        if let Err(e) = session.handshake() {                
            return Err(format!("SSH handshake error: {}", e));
        }
        
        if let Err(e) = session.userauth_password(user, password) {
            return Err(format!("Authentication error: {e}"));
        }

        assert!(session.authenticated());
        self.session = Some(session);
        self.host = host.to_string();
        self.user = user.to_string();
        self.password = password.to_string();
        Ok(())
    }
    pub fn connect_with_key(&mut self, 
        host: &str, port: i16, user: &str, pkey: &str) -> Result<(), String> {
        let tcp = match self._get_tcp(host, port) {
            Err(e) => return Err(e),
            Ok(o) => o,
        };
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);

        if let Err(e) = session.handshake() {
            return Err(format!("SSH handshake error: {}", e));
        }

        let private_key = std::path::Path::new(pkey);

        if let Err(e) = session.userauth_pubkey_file(user, None, private_key, None) {
            return Err(format!("Authentication error: {e}"));
        }

        assert!(session.authenticated());
        self.session = Some(session);
        self.host = host.to_string();
        self.user = user.to_string();
        self.private_key = pkey.to_string();
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
    const USER: &str = "support";
    const PASS: &str = "support";
    const HOST: &str = "localhost";
    const PORT: i16 = 22;

    #[test]
    fn connect_with_password() {
        let mut ssh = Ssh::new();
        let r = ssh.connect_with_password(HOST, PORT, USER, PASS);
        assert!(r.is_ok());
    }
    #[test]
    fn connect_with_password_wrong() {
        let mut ssh = Ssh::new();
        let r = ssh.connect_with_password(HOST, PORT, USER, "wrong");
        assert!(r.is_err());
    }
    #[test]
    fn connect_with_key() {
        let mut ssh = Ssh::new();
        let pkey = Ssh::private_key_path();
        let pkey = pkey.to_str().unwrap();
        let r = ssh.connect_with_key(HOST, PORT, USER, &pkey);
        assert!(r.is_ok());
    }
    #[test]
    fn connect_with_key_wrong() {
        let mut ssh = Ssh::new();
        let r = ssh.connect_with_key(HOST, PORT, USER, "/invalid/key");
        assert!(r.is_err());
    }
    #[test]
    fn connect_with_host_wrong() {
        let mut ssh = Ssh::new();
        let r = ssh.connect_with_password("example.com", PORT, USER, PASS);
        assert!(r.is_err());
    }
    #[test]
    fn run_command() {
        let mut ssh = Ssh::new();
        let r = ssh.connect_with_password(HOST, PORT, USER, PASS);
        assert!(r.is_ok());
        let output = ssh.run("whoami").unwrap();
        assert_eq!("support", output.as_str());
    }
    #[test]
    fn has_private_key() {
        assert!(Ssh::has_private_key());
    }
    #[test]
    #[ignore = "makes ssh keys unavailable for other tests to pass"]
    fn generate_keys() {
        let seckey = Ssh::private_key_path();
        let secbak = PathBuf::from(seckey.to_string_lossy().to_string() + ".bak");
        let pubkey = Ssh::public_key_path();
        let pubbak = PathBuf::from(pubkey.to_string_lossy().to_string() + ".bak");

        // backup keys
        if seckey.exists() {
            std::fs::rename(&seckey, &secbak).unwrap();
        }
        if pubkey.exists() {
            std::fs::rename(&pubkey, &pubbak).unwrap();
        }
        assert!(!Ssh::has_private_key());
        assert!(!Ssh::has_public_key());

        assert!(Ssh::generate_keys().is_ok()); 
        assert!(Ssh::generate_public_key().is_ok());  
        assert!(Ssh::has_private_key());
        assert!(Ssh::has_public_key());

        // restore keys
        if secbak.exists() {
            std::fs::rename(&secbak, &seckey).unwrap();
        }
        if pubbak.exists() {
            std::fs::rename(&pubbak, &pubkey).unwrap();
        }
        
    }
    
    #[test]
    fn setup_ssh() {
        assert!(Ssh::setup_ssh(HOST, PORT, USER, PASS).is_ok());
    }

    #[test]
    fn supported_algs() {
        println!("{}",Ssh::supported_algs());
    }
}