use crate::prelude::*;

use crate::things::get_things;
use crate::{args::Args, logic::choose_mode};

pub mod args;
pub mod logic;
pub mod prelude;
pub mod thetrait;
pub mod config;
pub mod common;
pub mod things;

pub struct Options {
    pub args: Args,
    pub config: Config,
    pub things: Vec<Box<dyn SetupThing>>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // The env is named RUST_LOG
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    let args = Args::parse_validate();
    let config = Config::load();
    let things = get_things();

    let options = Options {
        args, config, things
    };

    choose_mode(options);


    info!("All done, bye!");
    Ok(())
}
