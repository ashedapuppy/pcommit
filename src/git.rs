use crate::{lib::*, Arguments};
use anyhow::Result;
use git2::{Commit, Index, ObjectType, Repository, Status, Statuses};
use std::path::Path;

pub fn open_repository<P: AsRef<Path>>(path: P) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => panic!("failed to open current repository"),
    }
}

pub fn find_last_commit(repo: &Repository) -> Result<Commit> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let commit = obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    Ok(commit)
}

pub fn get_statuses(repo: &Repository) -> Statuses {
    match repo.statuses(None) {
        Ok(statuses) => statuses,
        Err(_) => panic!("failed to find file statuses of repository"),
    }
}

pub fn list_changed_files(repo: &Repository) -> Vec<String> {
    let statuses = get_statuses(repo);
    let mut list_changed_files: Vec<String> = Vec::new();
    statuses
        .iter()
        .filter(|e| e.status() != Status::CURRENT)
        .for_each(|entry| {
            if matches!(
                entry.status(),
                Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED
            ) {
                list_changed_files.push(entry.path().unwrap().to_owned());
            };
        });
    list_changed_files
}

pub fn add(args: &Arguments, repo: &Repository, index: &mut Index) -> Result<()> {
    let files_to_add = if args.add_all {
        list_changed_files(repo)
    } else {
        crate::input::files_to_add(repo)?
    };
    index.add_all(files_to_add, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn commit(repo: &Repository, commit: CommitMsg) -> Result<()> {
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(oid)?;
    let sig = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &commit.to_string(),
        &tree,
        &[&parent_commit],
    )?;
    Ok(())
}
