use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(
        short = 't',
        long = "target",
        help = "The directory to put the files into (Default: $HOME/Videos. Use the SORTER_TARGET_DIR env to change this.)"
    )]
    pub target: Option<PathBuf>,

    #[clap(
        short = 's',
        long = "src",
        help = "The directory to be used as a source for the files (Default: $HOME/Downloads. Use the SORTER_SRC_DIR env to change this.)"
    )]
    pub src: Option<PathBuf>,

    #[clap(
        short = 'e',
        long = "ext",
        help = "The extension for the files to be sorted (Default: mp4)"
    )]
    pub ext: Option<String>,
}
