use rustyline::{error::ReadlineError, Editor};
use std::process::exit;

pub fn get_type() -> String {
    let types = ["feat", "fix", "docs", "test", "refactor"];
    let mut rl = Editor::<()>::new();
    let read_type = rl.readline("type=> ");
    match read_type {
        Ok(line) if types.contains(&(line.as_str().trim())) => line,
        Ok(_) => get_type(),
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
            panic!();
        }
    }
}

pub fn get_body() -> Option<String> {
    let mut rl = Editor::<()>::new();
    let read_body = rl.readline("body(optional)=> ");
    match read_body {
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
            panic!();
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
            panic!();
        }
    }
}
