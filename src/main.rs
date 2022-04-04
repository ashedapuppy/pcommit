mod git;
mod input;
mod lib;

use clap::Parser;
use colored::*;

// git commit formatter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Disable git add
    #[clap(short = 'a', long = "no-add")]
    no_add: bool,

    /// Disable git push
    #[clap(short = 'p', long = "no-push")]
    no_push: bool,
}

fn main() {
    //! pcommit generates a commit message from user input according to
    //! [this](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13)
    //! commit message convention, it then adds modified files, creates the commit
    //! and calls git push
    //!
    //! # Usage:
    //! ./pcommit [-a/--no-add] [-p/--no-push] [-h/--help] [-V/--version]
    //!
    #![warn(missing_docs)]
    #![warn(missing_doc_code_examples)]
    let args = Arguments::parse();
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
