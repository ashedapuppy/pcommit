mod git;
mod input;
mod lib;

use anyhow::Result;
use clap::Parser;
use git2::Repository;

// git commit formatter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Add all changed files to commit
    #[clap(short = 'a', long = "all")]
    add_all: bool,
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

    let repo: Repository = git::open_repository(".");
    let mut index = repo.index()?;

    assert!(!git::list_changed_files(&repo).is_empty());

    let commit_full = lib::CommitMsg::new(input::get_type(), input::get_description(), None);
    git::add(args, &repo, &mut index)?;
    git::commit(&repo, commit_full)?;

    Ok(())
}
