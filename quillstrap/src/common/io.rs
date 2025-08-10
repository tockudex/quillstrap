use std::{
    env,
    path::{Path, PathBuf},
};

use crate::prelude::*;

pub fn read_file_str(path: String) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path.clone()) {
        Ok(x) => Ok(x),
        Err(x) => {
            error!("Failed to read file at path: \"{}\", error: {:?}", &path, x);
            Err(x)
        }
    }
}

pub fn remove_file(path: String) -> Result<(), std::io::Error> {
    match std::fs::remove_file(path.clone()) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Failed to remove file at path: \"{}\", error: {:?}",
                &path, e
            );
            Err(e)
        }
    }
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn dir_change(path: &str) {
    env::set_current_dir(path).expect(&format!("Failed to change directory to: {}", path))
}

pub fn mkdir_p(path: &str) {
    let dir = Path::new(path);
    std::fs::create_dir_all(dir).expect(&format!("Failed to create dir at: {}", path));
}

pub fn dir_current() -> String {
    env::current_dir().expect("Failed to get the current dir").to_str().unwrap().to_string()
}

pub fn set_var(key: &str, val: &str) {
    unsafe {
        env::set_var(key, val);
    }
}

