use std::process::{Command, Stdio};
use std::io;

pub fn run_cmd(command: &str, args: &[String]) -> io::Result<()> {
    #[cfg(not(target_os = "windows"))]
    {
        let mut complete_command = String::from(command);
        for arg in args {
            complete_command.push(' ');
            complete_command.push_str(arg);
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(complete_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.trim().is_empty() {
            print!("{}", stdout);
        }
        if !stderr.trim().is_empty() {
            print!("{}", stderr);
        }
    }

    #[cfg(target_os = "windows")]
    {
        let mut complete_command = String::from(command);

        for arg in args {
            complete_command.push(' ');
            complete_command.push_str(arg);
        }

        let output = Command::new("cmd")
            .arg("/c")
            .arg(complete_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.trim().is_empty() {
            print!("{}", stdout);
        }
        if !stderr.trim().is_empty() {
            print!("{}", stderr);
        }
    }

    Ok(())
}

pub fn flux_print(message: &str) {
    println!("Flux => {}", message);
}
