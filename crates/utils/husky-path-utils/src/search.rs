use std::fs;
use std::path::{Path, PathBuf};

/// Recursively searches for files under the given directory that match the predicate
pub fn find_files<P>(dir: impl AsRef<Path>, predicate: P) -> std::io::Result<Vec<PathBuf>>
where
    P: Fn(&Path) -> bool,
{
    let mut matches = Vec::new();
    find_files_aux(dir.as_ref(), &predicate, &mut matches)?;
    Ok(matches)
}

fn find_files_aux<P>(dir: &Path, predicate: &P, matches: &mut Vec<PathBuf>) -> std::io::Result<()>
where
    P: Fn(&Path) -> bool,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_files_aux(&path, predicate, matches)?;
            } else if predicate(&path) {
                matches.push(path);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_find_files() -> std::io::Result<()> {
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path();

        // Create test files
        File::create(base_path.join("test1.txt"))?;
        File::create(base_path.join("test2.rs"))?;

        let subdir = base_path.join("subdir");
        fs::create_dir(&subdir)?;
        File::create(subdir.join("test3.txt"))?;
        File::create(subdir.join("test4.rs"))?;

        // Test finding .txt files
        let txt_files = find_files(base_path, |p| {
            p.extension().and_then(|e| e.to_str()) == Some("txt")
        })?;
        assert_eq!(txt_files.len(), 2);

        // Test finding .rs files
        let rs_files = find_files(base_path, |p| {
            p.extension().and_then(|e| e.to_str()) == Some("rs")
        })?;
        assert_eq!(rs_files.len(), 2);

        Ok(())
    }
}
