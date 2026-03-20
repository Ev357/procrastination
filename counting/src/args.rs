use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub path: PathBuf,

    #[arg(short, long)]
    pub ignore: Option<Vec<String>>,
}
