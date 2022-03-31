use crate::lib::CommitMsg;
use clap::Parser;
use colored::*;

mod git;
mod input;
mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Disables git add
    #[clap(short = 'a', long = "no-add")]
    noadd: bool,

    /// Disables git push
    #[clap(short = 'p', long = "no-push")]
    nopush: bool,
}

fn main() {
    let args = Args::parse();
    println!(
        "commit types :\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        "feat".red(),
        "fix".blue(),
        "docs".yellow(),
        "refactor".green(),
        "test".magenta(),
    );
    let msg = CommitMsg::new(input::get_type(), input::get_desc(), input::get_body());
    if !args.noadd {
        git::add();
    }
    git::commit(msg);
    if !args.nopush {
        git::push();
    }
}
