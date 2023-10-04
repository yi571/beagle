use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;
mod init;
mod sniff;
mod list;
mod generate;
extern crate time;
pub mod hashing;
fn main() {
    let command_line_args = CommandLineArgs::from_args();

    match command_line_args.action {
        cli::Action::Init { path } => init::init(path),
        cli::Action::Sniff { message, path } => sniff::sniff(message, path),
        cli::Action::List { path } => list::get_csv_list(path),
        cli::Action::Generate { path, item, export_path } => {
            generate::generate(path, item, export_path)
        }
    }

}

