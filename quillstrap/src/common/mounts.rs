use crate::{prelude::*};

pub fn umount_recursive(target: &str) {
    if let Ok(mounts) = read_file_str("/proc/mounts".to_string()) {
        let mut dirs: Vec<&str> = mounts.lines()
            .filter_map(|line| line.split_whitespace().nth(1))
            .filter(|dir| dir.contains(target))
            .collect();
        dirs.sort_by(|a, b| b.cmp(a));
        for dir in dirs {
            let result = run_command(&format!("umount {}", dir), false);
            if result.is_err() {
                let _ = run_command(&format!("umount -n {}", dir), false);
            }
        }
    }
}

pub fn is_mount_point(path: &str) -> bool {
    let output = run_command_get_output(&format!("mountpoint {}", path));
    if output.contains("is a mountpoint") {
        return true;
    }
    false
}

pub fn mount_point(path: &str, mount_name: &str) {
    if !is_mount_point(path) {
        run_command(&format!("mount -t {} {} {}", mount_name, mount_name, path), false).expect("Failed to mount point");
    }
}