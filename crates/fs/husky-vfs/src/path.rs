use crate::*;

#[salsa::tracked]
pub fn path_class(db: &dyn VfsDb, path: PathBufItd) -> HuskyFileClass {
    // ad hoc
    HuskyFileClass::User
}
