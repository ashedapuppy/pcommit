use crate::lib::*;
use std::process::Command;

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
    let push_output = Command::new("git").arg("push").output();
    if push_output.is_err() {
        panic!("could not git push")
    };
}
