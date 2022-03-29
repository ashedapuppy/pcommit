use rustyline::{error::ReadlineError, Editor};
use std::fmt;
use std::process::{exit, Command};

pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Test,
    Refactor,
}

impl fmt::Display for CommitType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                CommitType::Feat => String::from("feat"),
                CommitType::Fix => String::from("fix"),
                CommitType::Docs => String::from("docs"),
                CommitType::Test => String::from("test"),
                CommitType::Refactor => String::from("refactor"),
            }
        )
    }
}

pub struct CommitMsg {
    pub commit_type: CommitType,
    pub description: String,
    pub body: Option<String>,
}

impl CommitMsg {
    #[must_use]
    pub fn new(commit_type: CommitType, description: String, body: Option<String>) -> Self {
        Self {
            commit_type,
            description,
            body,
        }
    }
}

impl fmt::Display for CommitMsg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.body {
            Some(body) => writeln!(
                fmt,
                "{}: {}\n\n{}",
                self.commit_type, self.description, body
            ),
            None => writeln!(fmt, "{}: {}\n", self.commit_type, self.description,),
        }
    }
}

pub fn git_add() {
    let add_output = Command::new("git").arg("add").arg(".").output();
    if add_output.is_err() {
        panic!("could not git add")
    };
}

pub fn git_commit(commit: CommitMsg) {
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit.to_string())
        .output();
    if commit_output.is_err() {
        panic!("could not git commit")
    };
}

pub fn git_push() {
    if push_y_or_n() {
        let push_output = Command::new("git").arg("push").output();
        if push_output.is_err() {
            panic!("could not git commit")
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

pub fn get_type() -> CommitType {
    let mut rl = Editor::<()>::new();
    let read_type = rl.readline("type=> ");
    match read_type {
        Ok(line) => match line.as_str() {
            "feat" => CommitType::Feat,
            "fix" => CommitType::Fix,
            "docs" => CommitType::Docs,
            "test" => CommitType::Test,
            "refactor" => CommitType::Refactor,
            _ => get_type(),
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

pub fn get_body() -> Option<String> {
    let mut rl = Editor::<()>::new();
    let read_type = rl.readline("body(optional)=> ");
    match read_type {
        Ok(line) => match line.as_str() {
            "" => None,
            s => Some(s.to_string()),
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
            exit(1);
        }
    }
}

pub fn get_desc() -> String {
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
