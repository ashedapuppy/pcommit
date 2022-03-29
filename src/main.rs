use colored::*;
use lib::{CommitMsg, CommitType};
use rustyline::{error::ReadlineError, Editor};
use std::process::{exit, Command};

mod lib;

fn git_add() {
    let add_output = Command::new("git").arg("add").arg(".").output();
    if add_output.is_err() {
        panic!("could not git add")
    };
}
fn git_commit(commit: CommitMsg) {
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit.to_string())
        .output();
    if commit_output.is_err() {
        panic!("could not git commit")
    };
}

fn get_type() -> CommitType {
    let mut rl = Editor::<()>::new();
    let commit_type: CommitType;
    let read_type = rl.readline("commit type=> ");
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
    commit_type
}

fn get_desc() -> String {
    let mut rl = Editor::<()>::new();
    let read_description = rl.readline("description=> ");
    match read_description {
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
    }
}

fn push_y_or_n() -> bool {
    let mut rl = Editor::<()>::new();
    let read_type = rl.readline("commit type=> ");
    match read_type {
        Ok(line) => match line.as_str() {
            "y" => true,
            "n" => false,
            _ => push_y_or_n(),
        },
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
}

fn git_push(do_b: bool) {
    if do_b {
        let push_output = Command::new("git").arg("push").output();
        if push_output.is_err() {
            panic!("could not git commit")
        };
    }
}

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
    let msg = CommitMsg::new(get_type(), get_desc(), None);
    git_add();
    git_commit(msg);
    git_push(push_y_or_n());
}
