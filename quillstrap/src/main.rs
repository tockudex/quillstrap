use crate::prelude::*;

use crate::{args::Args, logic::choose_mode};

pub mod args;
pub mod logic;
pub mod prelude;
pub mod thetrait;
pub mod config;
pub mod common;

fn main() -> Result<()> {
    color_eyre::install()?;

    // The env is named RUST_LOG
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    let args = Args::parse_validate();
    let config = Config::load();

    choose_mode(args);


    info!("All done, bye!");
    Ok(())
}
