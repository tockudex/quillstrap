use crate::prelude::*;
use std::process::{Command, Stdio};
use std::io;

pub fn run_command(command: &str, suppress: bool) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let mut child = Command::new(program)
            .args(&args)
            .stdout(if suppress { Stdio::null() } else { Stdio::inherit() })
            .stderr(if suppress { Stdio::null() } else { Stdio::inherit() })
            .spawn()?;

        child.wait()?;
    }
    Ok(())
}