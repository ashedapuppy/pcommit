use crate::lib::*;
use anyhow::Result;
use git2::{Commit, Index, ObjectType, Oid, Repository, Status, Statuses};

pub fn find_last_commit(repo: &Repository) -> Result<Commit> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let commit = obj.into_commit().map_err(|_| {
        error!("couldn't find last commit");
        git2::Error::from_str("Couldn't find commit")
    })?;
    Ok(commit)
}

pub fn get_statuses(repo: &Repository) -> Result<Statuses, git2::Error> {
    repo.statuses(None).map_err(|e| {
        error!("failed to find files statuses of repository");
        e
    })
}

pub fn list_of_changed_files(repo: &Repository) -> Result<Vec<String>> {
    let statuses = get_statuses(repo)?;
    let mut list_changed_files: Vec<String> = Vec::new();
    statuses
        .iter()
        .filter(|e| e.status() != Status::CURRENT)
        .for_each(|entry| {
            if matches!(
                entry.status(),
                Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED
            ) {
                list_changed_files.push(match entry.path() {
                    Some(path) => path,
                    None => {
                        error!("wrong path found in list of changed files");
                        ""
                    },
                }.to_owned());
            };
        });
    Ok(list_changed_files)
}

pub fn add(index: &mut Index, files_to_add: Vec<String>) -> Result<()> {
    index.add_all(files_to_add, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn commit(repo: &Repository, commit: CommitMsg, tag: Option<String>) -> Result<Oid> {
    // honestly, I wrote this without bothering to comment and now I don't understand
    // my own code because of how ugly the git2 api is...
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(oid)?;
    let sig = repo.signature()?;
    let oid = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &commit.to_string(),
        &tree,
        &[&parent_commit],
    )?;
    let commit = repo.find_object(oid, Some(ObjectType::Commit))?;
    if let Some(tag) = tag {
        return Ok(repo.tag_lightweight(tag.as_str(), &commit, true)?);
    }
    Ok(oid)
}
