#[allow(unused_imports)]
use crate::prelude::*;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(command: &str, show_output: bool) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let mut child = Command::new(program)
            .args(&args)
            .stdout(if !show_output {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stderr(if !show_output {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .spawn()?;

        child.wait()?;
    }
    Ok(())
}

pub fn run_shell_command(command: &str, show_output: bool) -> io::Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .stderr(if !show_output {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .spawn()?;

    child.wait()?;
    Ok(())
}

pub fn run_command_get_output(command: &str) -> String {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let output = Command::new(program)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        return format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::new()
}