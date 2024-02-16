use git2::{DiffFormat, DiffOptions, Error, Repository, StatusOptions};
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

    pub fn get_git_diff(&self, repo_path: &Path) -> Result<String, git2::Error> {
        let repo = Repository::open(repo_path)?;
        let head = repo.head()?;
        let tree = head.peel_to_tree()?;

        let mut opts = DiffOptions::new();
        let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), Some(&mut opts))?;

        let mut diff_string = String::new();
        diff.print(DiffFormat::Patch, |_, _, line| {
            let content = std::str::from_utf8(line.content()).unwrap_or_default();
            diff_string.push_str(content);
            true
        })?;

        Ok(diff_string)
    }

    pub fn git_commit(&self, msg: &str) -> Result<(), Error> {
        let repo = Repository::open(".")?;
        let sig = repo.signature()?;
        let mut index = repo.index()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let parent_commit = match repo.head() {
            Ok(reference) => match reference.target() {
                Some(target) => Some(repo.find_commit(target)?),
                None => None,
            },
            Err(_) => None,
        };
        let parents = parent_commit.iter().collect::<Vec<_>>();
        repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parents)?;
        Ok(())
    }
}
