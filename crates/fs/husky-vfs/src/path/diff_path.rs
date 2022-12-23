use super::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[salsa::interned(jar = VfsJar)]
pub struct DiffPath {
    #[return_ref]
    pub data: DiffPathBuf,
}

impl DiffPath {
    pub fn path<'a>(self, db: &'a dyn VfsDb) -> &'a Path {
        self.data(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DiffPathBuf(PathBuf);

// impl salsa::DebugWithDb<dyn VfsDb + '_> for DiffPathBuf {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//         db: &dyn VfsDb,
//         include_all_fields: bool,
//     ) -> std::fmt::Result {
//         let diff_path = todo!();
//         todo!()
//     }
// }

// impl<Db> salsa::DebugWithDb<Db> for DiffPathBuf
// where
//     Db: VfsDb,
// {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//         db: &Db,
//         include_all_fields: bool,
//     ) -> std::fmt::Result {
//         self.fmt(f, db as &dyn VfsDb, include_all_fields)
//     }
// }

#[test]
fn test_absolute_path_debug() {
    let db = DB::default();
    // let abs_path = DiffPath::new(path);
}

impl DiffPath {
    pub fn try_new(db: &dyn VfsDb, path: impl AsRef<Path>) -> VfsResult<Self> {
        Ok(Self::new(db, DiffPathBuf::try_new(db, path.as_ref())?))
    }
}

impl DiffPathBuf {
    pub fn try_new(db: &dyn VfsDb, path: &Path) -> VfsResult<Self> {
        let diff = |path: &Path| -> VfsResult<_> {
            pathdiff::diff_paths(path, db.vfs_cache().base_path()?).ok_or(VfsError::FailToDiff)
        };
        let diff_path = if path.is_absolute() {
            diff(path)
        } else {
            diff(
                &std::path::absolute(&path).map_err(|e| VfsError::FailToAbsolutize {
                    path: path.to_owned(),
                    error_message: e.to_string(),
                })?,
            )
        }?;
        Ok(DiffPathBuf(diff_path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for DiffPathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
