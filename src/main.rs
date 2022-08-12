mod git;
mod input;
mod lib;

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

    let repo: Repository = Repository::open(".")?;
    let mut index = repo.index()?;

    if list_of_changed_files(&repo).is_empty() {
        return Ok(())
    }

    let commit_full = lib::CommitMsg::new(input::get_type(), input::get_description(), None);
    git::add(args.add_all, &repo, &mut index)?;
    git::commit(&repo, commit_full, args.tag)?;

    if args.push {
        std::process::Command::new("git").arg("push").output()?;
    }

    Ok(())
}
