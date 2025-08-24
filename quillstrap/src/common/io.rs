use std::{
    env,
    path::Path,
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

pub fn copy_file(src: &str, dest: &str) -> Result<(), std::io::Error> {
    match std::fs::copy(&src, &dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Failed to copy file from \"{}\" to \"{}\", error: {:?}",
                &src, &dest, e
            );
            Err(e)
        }
    }
}

pub fn remove_file(path: &str) -> Result<(), std::io::Error> {
    match std::fs::remove_file(path) {
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
    env::current_dir()
        .expect("Failed to get the current dir")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn set_var(key: &str, val: &str) {
    unsafe {
        env::set_var(key, val);
    }
}

// With / at the end
pub fn get_path_of_thing(thing: &TraitWrapper, options: &crate::Options) -> String {
    format!(
        "{}{}/{}{}/",
        options.path_of_repo,
        MAIN_BUILD_DIR,
        thing.path(),
        thing.name()
    )
}
