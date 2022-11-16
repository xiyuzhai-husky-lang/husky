use crate::*;
use std::path::{Path, PathBuf};

impl Internable for PathBuf {
    type Ref<'a> = &'a Path;

    type Interned = PathItd;

    fn new_itr() -> crate::Interner<Self> {
        Interner::new_empty()
    }

    fn try_direct_from_ref<'a>(r: Self::Ref<'a>) -> Option<Self::Interned> {
        None
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Ref<'static> {
        todo!()
    }

    unsafe fn cast_to_static_ref<'a>(r: Self::Ref<'a>) -> Self::Ref<'static> {
        todo!()
    }

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        &self
    }

    fn new_itd(&'static self, idx: usize) -> Self::Interned {
        PathItd(InternedRefWrapper::new(&self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathItd(InternedRefWrapper<Path>);

impl std::ops::Deref for PathItd {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub type PathInterner = Interner<PathBuf>;

pub trait InternPath {
    fn path_itr(&self) -> &PathInterner;

    fn intern_path(&self, path: PathBuf) -> PathItd {
        self.path_itr()
            .intern(match std::fs::canonicalize(path.clone()) {
                Ok(path) => path,
                Err(_) => todo!(),
            })
    }
}
