use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use interner::{DefaultItd, Internable, InternedRefWrapper, Interner};
#[cfg(feature = "lsp_support")]
use lsp_types::Url;

pub struct HuskyFile(PathBuf);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileItd(InternedRefWrapper<Path>);
pub type FileInterner = Interner<HuskyFile>;

impl Internable for HuskyFile {
    type Borrowed<'a> = &'a Path;

    type BorrowedRaw = *const Path;

    type Interned = FileItd;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
        todo!()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        None
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static> {
        todo!()
    }

    fn to_borrowed<'a>(&'a self) -> Self::Borrowed<'a> {
        self.0.deref()
    }

    fn new_itd(&'static self, id: usize) -> Self::Interned {
        FileItd(InternedRefWrapper::new(&self.0))
    }
}

impl Deref for FileItd {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub trait InternFile {
    fn file_interner(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FileItd {
        self.file_interner()
            .intern(HuskyFile(match std::fs::canonicalize(path.clone()) {
                Ok(path) => path,
                Err(_) => path,
            }))
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

impl From<&str> for HuskyFile {
    fn from(value: &str) -> Self {
        HuskyFile(value.into())
    }
}
