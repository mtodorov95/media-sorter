use anime_sorter::{config::Config, opts::Opts, sorter::Sorter};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let sorter = Sorter::from_config(config);
    sorter.sort()?;

    return Ok(());
}
