use crate::*;
use interner::{
    path::{InternPath, PathInterner},
    Internable, InternedRefWrapper, Interner,
};
#[cfg(feature = "lsp_support")]
use lsp_types::Url;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

pub trait InternHuskyPath: InternPath {
    fn intern_module_path(&self, path: PathBuf) -> Option<ModulePathItd> {
        ModulePathItd::new(self.path_itr().intern(path))
    }

    #[cfg(feature = "lsp_support")]
    fn it_url(&self, url: &Url) -> Result<PathItd, ()> {
        Ok(self.intern_path(url.to_file_path()?))
    }
}

#[test]
fn test_intern_file() {
    use husky_check_utils::*;
    let interner: PathInterner = Default::default();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}
