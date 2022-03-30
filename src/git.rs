use rustyline::{error::ReadlineError, Editor};
use std::process::{exit, Command};

use crate::lib::*;

pub fn add() {
    let add_output = Command::new("git").arg("add").arg(".").output();
    if add_output.is_err() {
        panic!("could not git add")
    };
}

pub fn commit(commit: CommitMsg) {
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit.to_string())
        .output();
    if commit_output.is_err() {
        panic!("could not git commit")
    };
}

pub fn push() {
    if push_y_or_n() {
        let push_output = Command::new("git").arg("push").output();
        if push_output.is_err() {
            panic!("could not git push")
        };
    }
}

fn push_y_or_n() -> bool {
    let mut rl = Editor::<()>::new();
    let read_type = rl.readline("push?(y/n)=> ");
    match read_type {
        Ok(line) => match line.as_str() {
            "y" => true,
            "n" => false,
            _ => push_y_or_n(), // recursion is a banger
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
