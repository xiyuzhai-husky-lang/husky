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

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self.data(db) {
            LinktimeTargetPathData::Package(package_path) => package_path.toolchain(db),
            LinktimeTargetPathData::Workspace(_) => todo!(),
        }
    }

    /// returns absolute path
    pub fn rust_workspace_abs_dir<'a>(self, db: &'a ::salsa::Db) -> &'a std::path::Path {
        linktime_target_rust_abs_dir(db, self)
    }

    pub fn rust_workspace_rustfmt_toml_path<'a>(self, db: &'a ::salsa::Db) -> &'a std::path::Path {
        linktime_target_rust_workspace_rustfmt_toml_path(db, self)
    }

    pub fn rust_workspace_manifest_path<'a>(self, db: &'a ::salsa::Db) -> &'a std::path::Path {
        linktime_target_rust_workspace_manifest_path(db, self)
    }

    pub fn transpilation_setup(self, db: &::salsa::Db) -> TranspilationSetup {
        linktime_target_transpilation_setup(db, self)
    }
}

/// returns absolute path
#[salsa::tracked(jar = VfsJar, return_ref)]
fn linktime_target_rust_abs_dir(
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
fn linktime_target_rust_workspace_rustfmt_toml_path(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> std::path::PathBuf {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => package_path
            .dir(db)
            .expect("todo: should guarantee ok")
            .abs_path(db)
            .expect("todo: should guarantee ok")
            .join("target-rs/rustfmt.toml"),
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

#[salsa::tracked(jar = VfsJar)]
fn linktime_target_transpilation_setup(
    db: &::salsa::Db,
    path: LinktimeTargetPath,
) -> TranspilationSetup {
    TranspilationSetup::new(db, path, Some(RustTranspilationSetupData {}), None)
}

#[salsa::tracked(jar = VfsJar)]
pub struct TranspilationSetup {
    #[id]
    path: LinktimeTargetPath,
    #[return_ref]
    pub _rust_data: Option<RustTranspilationSetupData>,
    #[return_ref]
    pub _mlir_data: Option<MlirTranspilationSetupData>,
}

impl TranspilationSetup {
    pub fn rust_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a RustTranspilationSetupData> {
        todo!()
    }

    pub fn c_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a CTranspilationSetupData> {
        todo!()
    }

    pub fn llvm_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a LlvmTranspilationSetupData> {
        todo!()
    }

    pub fn mlir_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a MlirTranspilationSetupData> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RustTranspilationSetupData {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CTranspilationSetupData {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LlvmTranspilationSetupData {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MlirTranspilationSetupData {}
