use super::*;
use std::path::{Path, PathBuf};

#[salsa::interned(db = VfsDb, jar = VfsJar)]
pub struct DiffPath {
    #[return_ref]
    pub data: DiffPathBuf,
}

impl DiffPath {
    pub fn path<'a>(self, db: &'a dyn VfsDb) -> &'a Path {
        self.data(db)
    }

    pub fn abs_path(self, db: &dyn VfsDb) -> VfsResult<PathBuf> {
        std::path::absolute(db.vfs_cache().base_path()?.join(&self.data(db).0))
            .map_err(|_e| todo!())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DiffPathBuf(PathBuf);

#[test]
fn test_absolute_path_debug() {
    let _db = DB::default();
    // let abs_path = DiffPath::new(path);
}

impl DiffPath {
    // todo: room for optimization when path is owned
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
