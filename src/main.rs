mod git;
mod input;
mod lib;

use crate::lib::CommitMsg;

use clap::Parser;
use colored::*;

/// git commit formatter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Disable git add
    #[clap(short = 'a', long = "no-add")]
    no_add: bool,

    /// Disable git push
    #[clap(short = 'p', long = "no-push")]
    no_push: bool,
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
    if !args.no_add {
        git::add();
    }
    git::commit(msg);
    if !args.no_push {
        git::push();
    }
}
