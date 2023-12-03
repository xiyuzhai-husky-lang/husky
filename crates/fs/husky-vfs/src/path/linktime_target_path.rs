use super::*;
use crate::workspace_path::WorkspacePath;

#[salsa::interned(db = VfsDb, jar = VfsJar, constructor =new_inner)]
pub struct LinktimeTargetPath {
    pub data: LinktimeTargetPathData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = VfsDb, jar = VfsJar)]
pub enum LinktimeTargetPathData {
    Package(PackagePath),
    Workspace(WorkspacePath),
}

impl LinktimeTargetPath {
    pub fn new_package(package_path: PackagePath, db: &::salsa::Db) -> Self {
        Self::new_inner(db, LinktimeTargetPathData::Package(package_path))
    }

    pub fn rust_workspace_dir<'a>(self, db: &'a ::salsa::Db) -> &'a std::path::Path {
        linktime_target_rust_dir(db, self)
    }

    pub fn rust_workspace_manifest_path<'a>(self, db: &'a ::salsa::Db) -> &'a std::path::Path {
        linktime_target_rust_workspace_manifest_path(db, self)
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn linktime_target_rust_dir(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> std::path::PathBuf {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => package_path
            .dir(db)
            .expect("todo: should guarantee ok")
            .abs_path(db)
            .expect("todo: should guarantee ok")
            .join("target-rs"),
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn linktime_target_rust_workspace_manifest_path(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> std::path::PathBuf {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => package_path
            .dir(db)
            .expect("todo: should guarantee ok")
            .abs_path(db)
            .expect("todo: should guarantee ok")
            .join("target-rs/Cargo.toml"),
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}
