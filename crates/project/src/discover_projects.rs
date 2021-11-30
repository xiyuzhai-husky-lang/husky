use std::fs::{self, read_dir};

use rustc_hash::FxHashSet;

use common::error::*;
use vfs::{AbsPath, AbsPathBuf};

use crate::Project;

pub fn discover_projects(paths: &[AbsPathBuf]) -> Vec<Project> {
    let mut res = paths
        .iter()
        .filter_map(|it| discover(it.as_ref()).ok())
        .flatten()
        .collect::<FxHashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    res.sort();
    res
}

pub fn discover(path: &AbsPath) -> Result<Vec<Project>> {
    return match find_in_parent_dirs(path) {
        Some(project) => Ok(vec![project]),
        None => Ok(find_in_child_dirs(path)),
    };

    const MAIN_FILE_NAME: &str = "main.hsk";

    fn find_in_parent_dirs(path: &AbsPath) -> Option<Project> {
        if path.file_name().unwrap_or_default() == MAIN_FILE_NAME {
            return Project::new(path.to_path_buf()).ok();
        }

        let mut current = Some(path);

        while let Some(path) = current {
            let candidate = path.join(MAIN_FILE_NAME);
            if fs::metadata(&candidate).is_ok() {
                return Project::new(candidate).ok();
            }
            current = path.parent();
        }

        None
    }

    fn find_in_child_dirs(path: &AbsPath) -> Vec<Project> {
        if let Ok(entries) = read_dir(path) {
            // Only one level down to avoid cycles the easy way and stop a runaway scan with large projects
            entries
                .filter_map(Result::ok)
                .map(|it| it.path().join(MAIN_FILE_NAME))
                .filter(|candidate| candidate.exists())
                .map(AbsPathBuf::assert)
                .filter_map(|candidate| Project::new(candidate).ok())
                .collect()
        } else {
            vec![]
        }
    }
}
