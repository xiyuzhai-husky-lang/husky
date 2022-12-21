use super::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[salsa::interned(jar = VfsJar)]
pub struct AbsolutePath {
    #[return_ref]
    pub data: AbsolutePathBuf,
}

impl AbsolutePath {
    pub fn path<'a>(self, db: &'a dyn VfsDb) -> &'a Path {
        self.data(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AbsolutePathBuf(PathBuf);

// impl salsa::DebugWithDb<dyn VfsDb + '_> for AbsolutePath {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//         db: &dyn VfsDb,
//         include_all_fields: bool,
//     ) -> std::fmt::Result {
//         todo!()
//     }
// }

impl<Db> salsa::DebugWithDb<Db> for AbsolutePathBuf
where
    Db: VfsDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[test]
fn test_absolute_path_debug() {
    let db = DB::default();
    // let abs_path = AbsolutePath::new(path);
}

impl AbsolutePath {
    pub fn try_new(db: &dyn VfsDb, path: &Path) -> VfsResult<Self> {
        Ok(Self::new(db, AbsolutePathBuf::try_new(path)?))
    }
}

impl AbsolutePathBuf {
    pub fn try_new(path: &Path) -> VfsResult<Self> {
        Ok(AbsolutePathBuf(std::path::absolute(&path).map_err(
            |e| VfsError::FailToAbsolutize {
                path: path.to_owned(),
                error_message: e.to_string(),
            },
        )?))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for AbsolutePathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
