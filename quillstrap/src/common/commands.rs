#[allow(unused_imports)]
use crate::prelude::*;
use std::process::{Command, Stdio};
use std::io;

pub fn run_command(command: &str, show_output: bool) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let mut child = Command::new(program)
            .args(&args)
            .stdout(if !show_output { Stdio::null() } else { Stdio::inherit() })
            .stderr(if !show_output { Stdio::null() } else { Stdio::inherit() })
            .spawn()?;

        child.wait()?;
    }
    Ok(())
}