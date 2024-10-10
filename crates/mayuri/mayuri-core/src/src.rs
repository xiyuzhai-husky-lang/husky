use husky_sha_utils::Sha512Output;
use husky_sha_utils::ShaHash;
use std::fs;
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct MayuriSrc {
    dir_path: PathBuf,
    files: HashMap<String, MayuriSrcFile>,
}

#[derive(Debug, Clone)]
pub struct MayuriSrcFile {
    pub content: Arc<String>,
}

impl MayuriSrcFile {
    fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let content = Arc::new(fs::read_to_string(path)?);
        Ok(MayuriSrcFile { content })
    }
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
                let mayuri_code = MayuriSrcFile::from_file(path)?;
                let sha = mayuri_code.content.as_bytes().sha512();
                shas.insert(relative_path, mayuri_code);
            }
        }

        Ok(Self {
            dir_path,
            files: shas,
        })
    }
}

impl Index<&str> for MayuriSrc {
    type Output = MayuriSrcFile;

    fn index(&self, key: &str) -> &Self::Output {
        self.files.get(key).unwrap_or_else(|| {
            panic!(
                "Source file not found: {} in directory: {}",
                key,
                self.dir_path.display()
            )
        })
    }
}
