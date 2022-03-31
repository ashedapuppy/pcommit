use colored::*;

mod git;
mod input;
mod lib;

use crate::lib::CommitMsg;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // TODO: add a -h/--help and/or parse command line args
    println!(
        "commit types :\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        "feat".red(),
        "fix".blue(),
        "docs".yellow(),
        "refactor".green(),
        "test".magenta(),
    );
    let msg = CommitMsg::new(input::get_type(), input::get_desc(), input::get_body());
    git::add();
    git::commit(msg);
    git::push();
}
