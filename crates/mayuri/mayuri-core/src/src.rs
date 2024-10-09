use husky_sha_utils::Sha512Output;
use husky_sha_utils::ShaHash;
use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct MayuriCode {
    pub sha: Sha512Output,
    pub content: String,
}

#[derive(Debug)]
pub struct MayuriSrc {
    dir_path: PathBuf,
    shas: HashMap<String, MayuriCode>,
}

impl MayuriSrc {
    pub fn new(dir_path: PathBuf, allowed_exts: &[&str]) -> Result<Self, std::io::Error> {
        let mut shas = HashMap::new();

        for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| allowed_exts.contains(&ext))
            {
                let relative_path = path
                    .strip_prefix(&dir_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let content = fs::read_to_string(path)?;
                let sha = content.as_bytes().sha512();
                shas.insert(relative_path, MayuriCode { sha, content });
            }
        }

        Ok(Self { dir_path, shas })
    }
}

impl Index<&str> for MayuriSrc {
    type Output = MayuriCode;

    fn index(&self, key: &str) -> &Self::Output {
        self.shas.get(key).unwrap_or_else(|| {
            panic!(
                "Source file not found: {} in directory: {}",
                key,
                self.dir_path.display()
            )
        })
    }
}
