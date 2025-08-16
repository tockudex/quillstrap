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

pub fn get_disk_partitions(disk: &str) -> Vec<String> {
    let mut partitions = Vec::new();

    let output = match Command::new("lsblk")
        .arg("-ln")
        .arg("-o")
        .arg("NAME")
        .arg(disk)
        .output()
    {
        Ok(output) => output,
        Err(_) => return partitions,
    };

    if !output.status.success() {
        return partitions;
    }

    let lines = String::from_utf8_lossy(&output.stdout);
    for line in lines.lines().skip(1) {
        // Skip the disk itself
        let name = line.trim();
        if !name.is_empty() {
            partitions.push(format!("/dev/{}", name));
        }
    }

    partitions
}

// expects /dev/sdaX or similar
pub fn get_partition_label(partition: &str) -> String {
    let name = Path::new(partition)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(partition);
    let uevent_path = format!("/sys/class/block/{}/uevent", name);
    let Ok(content) = std::fs::read_to_string(&uevent_path) else {
        return String::new();
    };
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("PARTNAME=") {
            return rest.to_string();
        }
    }
    String::new()
}

// outputs /dev/sda2
pub fn get_partition(label: &str) -> String {
    let path = format!("/dev/disk/by-partlabel/{}", label);
    std::fs::canonicalize(path)
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap()
}

use regex::Regex;
// Receives /dev/sda2
pub fn get_disk_part_numb(partition: &str) -> (String, u16) {
    let re = Regex::new(r"^(/dev/[a-zA-Z0-9]+(?:\d+)?(?:n\d+)?)(\d+)$").unwrap();
    re.captures(partition)
        .and_then(|cap| {
            let disk = cap.get(1)?.as_str().trim_end_matches('p').to_string();
            let num = cap.get(2)?.as_str().parse::<u16>().unwrap();
            Some((disk, num))
        })
        .unwrap()
}

// accepts labels
pub fn remove_partition(label: &str) {
    let (disk, part_number) = get_disk_part_numb(&get_partition(label));
    show_wait_toast(&format!(
        "Remove partition of label: {} number: {} in disk: {}",
        label, part_number, disk
    ));
    run_command(&format!("parted {} -s rm {}", disk, part_number), true).unwrap();
}

// mb are *1000 from gb!
// only ext4
pub fn resize_partition(label: &str, size_mb: u16) {
    let (disk, part_number) = get_disk_part_numb(&get_partition(label));
    show_wait_toast(&format!(
        "Resize partition of label: {} number: {} in disk: {} to size {}M",
        label, part_number, disk, size_mb
    ));

    run_command(
        &format!("e2fsck -f -y -v -C 0 {}", get_partition(label)),
        true,
    )
    .unwrap();
    run_command(
        &format!("parted -s {} resizepart {} {}M", disk, part_number, size_mb),
        true,
    )
    .unwrap();
    run_command(
        &format!("e2fsck -f -y -v -C 0 {}", get_partition(label)),
        true,
    )
    .unwrap();
    run_command(&format!("resize2fs -p {}", get_partition(label)), true).unwrap();
}
