use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(short = 't', long = "target")]
    pub target: Option<PathBuf>,

    #[clap(short = 's', long = "src")]
    pub src: Option<PathBuf>,

    #[clap(short = 'e', long = "ext")]
    pub ext: Option<String>,
}

