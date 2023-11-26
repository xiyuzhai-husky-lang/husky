use super::*;
use std::path::{Path, PathBuf};

/// `VirtualPath` is the path relative to the current dir of the current program,
/// it's guaranteed that equivalent paths are interned to the same id
#[salsa::interned(db = VfsDb, jar = VfsJar)]
pub struct VirtualPath {
    #[return_ref]
    _data: RelPathBuf,
}

impl VirtualPath {
    #[inline(always)]
    pub fn data<'a>(self, db: &'a Db) -> &'a Path {
        self._data(db)
    }

    pub fn abs_path(self, db: &::salsa::Db) -> VfsResult<PathBuf> {
        std::path::absolute(db.vfs_cache().current_dir().join(&self.data(db))).map_err(|_e| todo!())
    }

    pub fn file(self, db: &::salsa::Db) -> VfsResult<File> {
        db.file_from_virtual_path(self)
    }

    pub fn text<'a>(self, db: &'a Db) -> VfsResult<Option<&'a str>> {
        let file = self.file(db)?;
        Ok(file.text(db)?)
    }

    pub fn text_expected<'a>(self, db: &'a Db) -> VfsResult<&'a str> {
        let file = self.file(db)?;
        file.text(db)?.ok_or(VfsError::FileNotExists(self))
    }
}

impl VirtualPath {
    // todo: room for optimization when path is owned
    pub fn try_new(db: &::salsa::Db, path: impl AsRef<Path>) -> VfsResult<Self> {
        Ok(Self::new(db, RelPathBuf::try_new(db, path.as_ref())?))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RelPathBuf(PathBuf);

#[test]
fn test_absolute_path_debug() {
    let _db = DB::default();
    // let abs_path = VirtualPath::new(path);
}

impl RelPathBuf {
    pub fn try_new(db: &::salsa::Db, path: &Path) -> VfsResult<Self> {
        let diff = |path: &Path| -> VfsResult<_> {
            pathdiff::diff_paths(path, db.vfs_cache().current_dir()).ok_or(VfsError::FailToDiff)
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
        Ok(RelPathBuf(diff_path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for RelPathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
