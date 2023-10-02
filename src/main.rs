use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;
mod init;
mod sniff;
extern crate time;
pub mod hashing;
fn main() {
    let command_line_args = CommandLineArgs::from_args();

    match command_line_args.action {
        cli::Action::Init { path } => init::init(path),
        cli::Action::Sniff { message, path } => sniff::record(message, path),
        
    }

}

