use colored::*;
use lib::*;

mod lib;

fn main() {
    // TODO: add a -h/--help
    println!(
        "commit types :\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        "feat".clear().red(),
        "fix".blue(),
        "docs".yellow(),
        "refactor".green(),
        "test".magenta(),
    );
    // TODO: parse the commit body
    let msg = CommitMsg::new(get_type(), get_desc(), get_body());
    git_add();
    git_commit(msg);
    git_push();
}
