mod git;
mod input;
mod lib;

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
    // we get a user input, which can be None for the commit body,
    // so the builder pattern is not necessary
    let msg = lib::CommitMsg::new(input::get_type(), input::get_desc(), input::get_body());

    if !args.no_add {
        git::add();
    }
    git::commit(msg);
    if !args.no_push {
        git::push();
    }
}
