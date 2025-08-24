use crate::prelude::*;

// maybe use request crate in the future, im lazy
pub fn download_file(url: &str, output_file: &str) {
    let err = &format!("Failed to download from url: {}", url);
    run_command(&format!("wget -O {} {} -q --show-progress", output_file, url), true)
        .expect(err);

    if !path_exists(output_file) {
        panic!("{}", err);
    }
}
