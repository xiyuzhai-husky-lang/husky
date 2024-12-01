use crate::*;
use std::path::PathBuf;

#[eterned::eterned]
pub struct LxFilePath {
    #[return_ref]
    data: PathBuf,
}

impl std::fmt::Debug for LxFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = eterned::db::attached_interner_db().ok_or(std::fmt::Error)?;
        f.debug_tuple("LxFilePath").field(self.data(db)).finish()
    }
}
