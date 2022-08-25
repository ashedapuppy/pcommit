mod git;
mod input;
mod lib;

#[macro_use]
extern crate log;
extern crate simplelog;
use std::fs::File;

use simplelog::*;

use anyhow::Result;
use clap::Parser;
use git2::Repository;

use crate::git::list_of_changed_files;

// git commit formatter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Add all changed files to commit
    #[clap(short = 'a', long = "all")]
    add_all: bool,

    /// call git push after execution
    #[clap(short = 'p', long = "push")]
    push: bool,

    /// set a tag on the commit
    #[clap(short = 't', long = "tag")]
    tag: Option<String>,
}

fn main() -> Result<()> {
    //! pcommit generates a commit message from user input according to
    //! [this](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13)
    //! commit message convention, it then adds modified files and creates the commit
    //!
    //! # Usage:
    //! ./pcommit [-a/--all] [-h/--help] [-V/--version]
    //!
    let args = Arguments::parse();

    // setup logging
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("pcommit.log")?,
        ),
    ])?;

    let repo: Repository = Repository::open(".")?;
    let mut index = repo.index()?;

    let files_to_add = if args.add_all {
        list_of_changed_files(&repo)?
    } else {
        crate::input::files_to_add(&repo)?
    };

    if files_to_add.is_empty() {
        println!("no file changes to be committed, exiting...");
        return Ok(());
    }

    let commit_full = lib::CommitMsg::new(input::get_type(), input::get_description(), None);
    git::add(&mut index, files_to_add)?;
    git::commit(&repo, commit_full, args.tag)?;

    if args.push {
        std::process::Command::new("git").arg("push").output()?;
    }

    Ok(())
}
