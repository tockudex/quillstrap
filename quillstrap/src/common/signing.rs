use crate::prelude::*;

pub fn get_private_key_path(options: &Options) -> String {
    format!(
        "{}{}",
        options.path_of_repo, options.config.main_private_key_path
    )
}

pub fn generate_public_key(path: &str, options: &Options) {
    let err = format!("Failed to generate public key in: {}", path);
    run_command(
        &format!(
            "openssl rsa -in {} -out {} -outform PEM -pubout",
            get_private_key_path(options),
            path
        ),
        false,
    )
    .expect(&err);
    if !path_exists(path) {
        panic!("{}", &err);
    }
}
