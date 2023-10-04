use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// init
    #[structopt()]
    Init {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// sniff file diff
    Sniff {
        /// sniff file message
        #[structopt(short, long)]
        message: String,

        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// get records
    List {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// generate record
    Generate {
        /// folder path
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,

        /// item
        #[structopt(short, long)]
        item: String,

        /// export folder path
        #[structopt(short, long, parse(from_os_str))]
        export_path: Option<PathBuf>,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "check file hash", about = "calculate and record file hash")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
