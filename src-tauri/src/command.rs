use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn run(cmd: &str) -> (String, String, i32) {
    let r = if cfg!(windows) {
        Command::new("cmd").arg("/c").raw_arg(cmd).output().unwrap()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output().unwrap()
    };
    let stdout = String::from_utf8_lossy(&r.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&r.stderr).trim().to_string();
    (stdout, stderr, r.status.code().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
      
    #[test]
    fn run_command() {
        let (o,e,r) = run("echo hello");
        assert_eq!(o, "hello");
        assert_eq!(e, "");
        assert_eq!(r, 0);
    }
    #[test]
    fn run_stderr() {
        let (_,e,r) = run("dir c:\\nofile");
        println!("{e}");
        assert!(e.contains("Not Found"));
        assert_eq!(r, 1);
    }
    #[test]
    fn run_status() {
        let (o,e,r) = run("exit 1");
        assert_eq!(o, "");
        assert_eq!(e, "");
        assert_eq!(r, 1);
    }
    
    #[test]
    fn run_cancel_stdout() {
        let (o,e,r) = run("echo hello >nul");
        assert_eq!(o, "");
        assert_eq!(e, "");
        assert_eq!(r, 0);

    }
    #[test]
    fn run_cancel_stderr() {
        let (o,e,r) = run("dir c:\\nofile 2>&1");
        assert!(o.contains("Not Found"));
        assert_eq!(e, "");
        assert_eq!(r, 1);

    }
    
}

