use std::{thread, time};

pub mod cli;
pub mod commands;
pub mod git;
pub mod io;
pub mod serial;
pub mod deploy;

// TODO: make this show on the cli as a animation, then it dissapears
pub fn sleep_millis(ms: u64) {
    let ten_millis = time::Duration::from_millis(ms);
    thread::sleep(ten_millis);
}
