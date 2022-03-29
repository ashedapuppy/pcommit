use colored::*;
use lib::{CommitMsg, CommitType};
use rustyline::{error::ReadlineError, Editor};
use std::process::{exit, Command};

mod lib;

fn git_commit(commit: CommitMsg) {
    let add_output = Command::new("git").arg("add").arg(".").output();
    if add_output.is_err() {
        panic!("could not git add")
    }
    let _commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit.to_string())
        .output();
}

fn main() {
    let mut rl = Editor::<()>::new();

    println!(
        "commit types :\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        "feat".clear().red(),
        "fix".blue(),
        "docs".yellow(),
        "refactor".green(),
        "test".magenta(),
    );

    let commit_type: CommitType;
    let read_type = rl.readline("commit type>> ");
    match read_type {
        Ok(line) => {
            match line.as_str() {
                "feat" => commit_type = CommitType::Feat,
                "fix" => commit_type = CommitType::Fix,
                "docs" => commit_type = CommitType::Docs,
                "test" => commit_type = CommitType::Test,
                "refactor" => commit_type = CommitType::Refactor,
                _ => {
                    eprintln!("commit type not found");
                    exit(0);
                }
            };
        }
        Err(ReadlineError::Interrupted) => {
            eprintln!("CTRL-C");
            exit(0);
        }
        Err(ReadlineError::Eof) => {
            eprintln!("CTRL-D");
            exit(0);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(0);
        }
    }

    let read_description = rl.readline("description>> ");
    let commit_desc: String = match read_description {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            eprintln!("CTRL-C");
            exit(0);
        }
        Err(ReadlineError::Eof) => {
            eprintln!("CTRL-D");
            exit(0);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(0);
        }
    };

    // TODO: parse the commit body
    let msg = CommitMsg::new(commit_type, commit_desc, None);
    git_commit(msg);
}
