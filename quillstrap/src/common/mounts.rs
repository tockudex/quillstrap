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