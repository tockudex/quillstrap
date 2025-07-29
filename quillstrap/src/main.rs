use anyhow::Result;
use clap::Parser;
use log::info;

#[derive(Parser)]
#[command(about = "Quill OS build and bootstrap tool")]
pub struct Args {

}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    info!("All done!");
    Ok(())
}
