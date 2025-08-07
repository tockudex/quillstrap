use crate::prelude::*;

pub fn get_private_key_path(options: &Options) -> String {
    format!(
        "{}{}",
        options.path_of_repo, options.config.main_private_key_path
    )
}
