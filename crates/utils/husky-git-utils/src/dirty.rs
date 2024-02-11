use git2::{Repository, StatusOptions, StatusShow};
use husky_path_utils::rust::husky_cargo_workspace_manifest_dir;
use std::path::Path;

pub fn is_git_dir_dirty<P: AsRef<Path>>(path: P) -> Result<bool, git2::Error> {
    // Attempt to open the repository at the given path
    let repo = Repository::open(path)?;

    // Create a new vector to hold the options for status.
    let mut opts = StatusOptions::new();

    // Set the options to include untracked files and to show the index and working directory changes.
    opts.include_untracked(true)
        .show(StatusShow::IndexAndWorkdir);

    // Check the status of files in the repository
    let statuses = repo.statuses(Some(&mut opts))?;

    // If there are no statuses, the directory is not dirty
    Ok(!statuses.is_empty())
}

pub fn is_husky_git_dir_dirty() -> Result<bool, git2::Error> {
    is_git_dir_dirty(husky_cargo_workspace_manifest_dir())
}

#[track_caller]
pub fn assert_husky_git_dir_clean() {
    assert!(
        !is_git_dir_dirty(husky_cargo_workspace_manifest_dir()).unwrap(),
        "expect husky dir to be clean"
    )
}

#[ignore]
#[test]
fn main() {
    match is_husky_git_dir_dirty() {
        Ok(dirty) if dirty => println!("Git directory is dirty"),
        Ok(_) => println!("Git directory is clean"),
        Err(e) => println!("Error checking Git directory: {}", e),
    }
}
