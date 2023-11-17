use husky_vfs::{linktime_target_path::LinktimeTargetPath, workspace_path::WorkspacePath};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) enum RustTranspilationDep {
    Library(RustTranspilationLibraryDep),
    Registry(RustTranspilationRegistryDep),
    Local(RustTranspilationLocalDep),
}

impl RustTranspilationDep {
    pub(crate) fn transpile_to_fs(&self, db: &dyn RustTranspilationDb) {
        match self {
            RustTranspilationDep::Library(slf) => slf.transpile_to_fs(db),
            RustTranspilationDep::Registry(slf) => slf.transpile_to_fs(db),
            RustTranspilationDep::Local(slf) => slf.transpile_to_fs(db),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationLibraryDep {
    package_path: PackagePath,
}

impl RustTranspilationLibraryDep {
    pub(crate) fn transpile_to_fs(&self, db: &dyn RustTranspilationDb) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationRegistryDep {
    package_path: PackagePath,
}

impl RustTranspilationRegistryDep {
    pub(crate) fn transpile_to_fs(&self, db: &dyn RustTranspilationDb) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationLocalDep {
    package_path: PackagePath,
}

impl RustTranspilationLocalDep {
    pub(crate) fn transpile_to_fs(&self, db: &dyn RustTranspilationDb) {
        todo!()
    }
}

pub(crate) fn rust_transpilation_deps(
    target_path: LinktimeTargetPath,
    db: &dyn RustTranspilationDb,
) -> &[RustTranspilationDep] {
    match target_path {
        LinktimeTargetPath::Package(package_path) => {
            package_rust_transpilation_deps(db, package_path)
        }
        LinktimeTargetPath::Workspace(workspace_path) => {
            workspace_rust_transpilation_deps(db, workspace_path)
        }
    }
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_rust_transpilation_deps(
    db: &dyn RustTranspilationDb,
    package_path: PackagePath,
) -> Vec<RustTranspilationDep> {
    todo!()
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn workspace_rust_transpilation_deps(
    db: &dyn RustTranspilationDb,
    workspace_path: WorkspacePath,
) -> Vec<RustTranspilationDep> {
    todo!()
}
