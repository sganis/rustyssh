use std::process::Command;

pub fn run(cmd: &str, args: &str) -> Result<String, String> {
    println!("CMD: {cmd} {args}");
    let r = Command::new(cmd).arg(args).output().unwrap();                    
    let stdout = String::from_utf8(r.stdout).unwrap().trim().to_string();
    let stderr = String::from_utf8(r.stderr).unwrap().trim().to_string();
    if !r.status.success() {
        return Err(stderr);
    }
    Ok(stdout)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_run() {
        assert!(run("echo","hello").is_ok());
    }
    #[test]
    fn cmd_run_stdout() {
        let output = run("echo","hello").unwrap();
        assert_eq!(output, "hello".to_string());
    }
    #[test]
    fn cmd_run_stderr() {
        assert!(run("dir","F:/nofile").err().unwrap()
            .contains("No such file"));
    }
    #[test]
    fn cmd_run_pipe() {
        assert!(run("cmd","/c echo hello|findstr h").unwrap()
            .contains("hello"));
    }
    #[test]
    fn cmd_run_pipe_error() {
        assert_eq!(run("cmd","/c echo hello|findstr no").err().unwrap(),"");
    }

    
}