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
        num_args = 1..,
        help = "The extensions for the files to be sorted as a space separated list (Default: mp4)"
    )]
    pub ext: Option<Vec<String>>,

    #[clap(
        short = 'k',
        help = "Keeps file prefixes and does not rename the files"
    )]
    pub keep: bool,
}
