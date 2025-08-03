use crate::prelude::*;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(about = "Quill OS build and bootstrap tool")]
pub struct Args {
    #[arg(
        short,
        long,
        help = "You need to specify everything what quillstrap should do manually.",
        default_value_t = false
    )]
    pub manual_mode: bool,
    #[arg(
        short,
        long,
        help = "Specify a single action, then everything that's needed will be done to achieve it",
        default_value_t = false
    )]
    pub auto_mode: bool,
    #[arg(
        short,long,
        help = "Things to get (or check for updates), seperated by space",
        num_args = 1..,
    )]
    // Order is important in those vecs!
    pub get: Vec<String>,
    #[arg(
        short,long,
        help = "Things to build, seperated by space",
        num_args = 1..,
    )]
    pub build: Vec<String>,
    #[arg(
        short,long,
        help = "Things to clean, seperated by space",
        num_args = 1..,
    )]
    pub clean: Vec<String>,
    #[arg(
        short,long,
        help = "Things to deploy, seperated by space",
        num_args = 1..,
    )]
    pub deploy: Vec<String>,
    #[arg(short, long, help = "A single thing to run")]
    pub run: Option<String>,
}

impl Args {
    pub fn parse_validate() -> Self {
        let args = Args::parse();
        debug!("Initial args structure: {:#?}", args);

        // TODO: this is cool https://crates.io/crates/human-panic
        if args.auto_mode && args.manual_mode {
            panic!("Select only one mode!");
        }

        if !args.auto_mode && !args.manual_mode {
            panic!("Select a mode!");
        }

        if args.build.is_empty()
            && args.get.is_empty()
            && args.clean.is_empty()
            && args.deploy.is_empty()
            && args.run.is_none()
        {
            panic!("No action selected to be done!");
        }

        args
    }
}
