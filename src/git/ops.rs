use git2::{Error, Repository, StatusOptions};
use std::path::Path;

pub struct Commit {}

impl Commit {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_changes(&self, repo_path: &Path) -> Result<String, git2::Error> {
        let repo = Repository::open(repo_path)?;
        let mut options = StatusOptions::new();
        options.include_untracked(true);

        let statuses = repo.statuses(Some(&mut options))?;

        let mut changes = String::new();
        for entry in statuses.iter() {
            let status = entry.status();

            if status.is_wt_modified() {
                changes.push_str(&format!("Modified: {}\n", entry.path().unwrap_or_default()));
            } else if status.is_wt_new() {
                changes.push_str(&format!("New: {}\n", entry.path().unwrap_or_default()));
            } else if status.is_wt_deleted() {
                changes.push_str(&format!("Deleted: {}\n", entry.path().unwrap_or_default()));
            }
        }

        Ok(changes)
    }

    pub fn git_commit(&self, msg: &str) -> Result<(), Error> {
        let repo = Repository::open(".")?;
        let sig = repo.signature()?;
        let mut index = repo.index()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let parent_commit = match repo.head() {
            Ok(reference) => Some(repo.find_commit(reference.target().unwrap())?),
            Err(_) => None,
        };
        let parents = parent_commit.iter().collect::<Vec<_>>();
        repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parents)?;
        Ok(())
    }
}
