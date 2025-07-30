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
            error!("Failed to remove file at path: \"{}\", error: {:?}", &path, e);
            Err(e)
        }
    }
}