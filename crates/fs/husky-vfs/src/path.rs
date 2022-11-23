use crate::*;

#[salsa::tracked(jar = VfsJar)]
pub fn path_class(db: &dyn VfsDb, path: PathBufItd) -> HuskyFileClass {
    // ad hoc
    HuskyFileClass::User
}
