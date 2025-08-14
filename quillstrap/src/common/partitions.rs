use std::process::Command;

use crate::prelude::*;

pub fn choose_disk() -> String {
    let output = Command::new("lsblk")
        .arg("-ndo")
        .arg("NAME,TYPE")
        .output()
        .expect("Failed to run lsblk");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let disks: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == "disk" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    if disks.is_empty() {
        println!("No disks found!");
        return "".to_string();
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a disk")
        .default(0)
        .items(&disks)
        .interact()
        .unwrap();

    format!("/dev/{}", disks[selection].clone())
}