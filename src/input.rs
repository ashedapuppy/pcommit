use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

use crate::lib::CommitType;
use anyhow::Result;

pub fn files_to_add<T>(addlist: &[T]) -> Result<Vec<&str>> 
where T: AsRef<str> + std::fmt::Display {
    let defaults = vec![true; addlist.len()];
    let mut addedlist: Vec<&str> = vec![];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick files to add to commit (space to select)")
        .items(addlist)
        .defaults(&defaults[..])
        .interact()?;

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            let path = &addlist[selection];
            addedlist.push(path.as_ref());
            println!("  {}", addlist[selection]);
        }
    }
    Ok(addedlist)
}

pub fn get_type() -> CommitType {
    let types = ["feat", "fix", "docs", "test", "refactor"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("commit type")
        .items(&types)
        .default(0)
        .interact_on_opt(&Term::stderr());
    let commit_type = if let Ok(Some(index)) = selection {
        types[index]
    } else {
        ""
    };

    match commit_type {
        "feat" => CommitType::Feat,
        "fix" => CommitType::Fix,
        "docs" => CommitType::Docs,
        "test" => CommitType::Test,
        "refactor" => CommitType::Refactor,
        _ => get_type(),
    }
}

pub fn get_description() -> String {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("commit description")
        .interact_text();
    match input {
        Ok(string) => string,
        Err(_) => get_description(),
    }
}
