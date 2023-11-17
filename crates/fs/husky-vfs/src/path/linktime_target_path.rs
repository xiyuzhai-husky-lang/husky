use super::*;
use crate::workspace_path::WorkspacePath;

#[salsa::interned(db = VfsDb, jar = VfsJar, constructor =new_inner)]
pub struct LinktimeTargetPath {
    pub data: LinktimeTargetPathData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = VfsDb)]
pub enum LinktimeTargetPathData {
    Package(PackagePath),
    Workspace(WorkspacePath),
}

impl LinktimeTargetPath {
    pub fn new_package(package_path: PackagePath, db: &dyn VfsDb) -> Self {
        Self::new_inner(db, LinktimeTargetPathData::Package(package_path))
    }

    pub fn rust_dir<'a>(self, db: &'a dyn VfsDb) -> &'a std::path::Path {
        linktime_target_rust_dir(db, self)
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn linktime_target_rust_dir(db: &dyn VfsDb, target_path: LinktimeTargetPath) -> std::path::PathBuf {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => package_path
            .dir(db)
            .expect("todo: should guarantee ok")
            .data(db)
            .join("target-rs"),
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}
