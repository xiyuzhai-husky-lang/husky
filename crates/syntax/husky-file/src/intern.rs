use std::path::{Path, PathBuf};

use interner::{DefaultInternedPtr, Interner};
#[cfg(feature = "lsp_support")]
use lsp_types::Url;

pub type FileItd = DefaultInternedPtr<Path, PathBuf>;
pub type FileInterner = Interner<FileItd>;

pub trait InternFile {
    fn file_interner(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FileItd {
        self.file_interner()
            .intern(match std::fs::canonicalize(path.clone()) {
                Ok(path) => path,
                Err(_) => path,
            })
    }

    #[cfg(feature = "lsp_support")]
    fn it_url(&self, url: &Url) -> Result<FileItd, ()> {
        Ok(self.intern_file(url.to_file_path()?))
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::new_empty()
}

#[test]
fn test_intern_file() {
    use husky_check_utils::*;
    let interner = new_file_interner();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}
