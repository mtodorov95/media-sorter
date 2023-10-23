use anyhow::Result;
use clap::Parser;
use media_sorter::{config::Config, opts::Opts, sorter::Sorter};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let sorter = Sorter::from_config(config);
    sorter.sort()?;

    return Ok(());
}
