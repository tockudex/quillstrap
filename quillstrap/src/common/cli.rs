use crate::prelude::*;

// TODO: make this look better as in no (y/n)
pub fn show_wait_toast(str: &str) {
    let w = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(str)
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();

    if !w {
        panic!("Well, sadly, we cannot continue without it");
    }
}
