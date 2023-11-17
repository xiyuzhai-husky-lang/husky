use husky_manifest::{HasPackageManifest, ManifestDependency};
use husky_vfs::{
    linktime_target_path::LinktimeTargetPath, workspace_path::WorkspacePath, PackagePathSource,
};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) enum RustTranspilationDependency {
    Library(RustTranspilationLibraryDep),
    Registry(RustTranspilationRegistryDep),
    Local(RustTranspilationLocalDep),
}

impl RustTranspilationDependency {
    fn from_manifest(dep: &ManifestDependency, db: &dyn RustTranspilationDb) -> Self {
        let package_path = dep.package_path();
        match package_path.data(db) {
            PackagePathSource::Library => {
                RustTranspilationDependency::Library(RustTranspilationLibraryDep { package_path })
            }
            PackagePathSource::Registry { .. } => {
                RustTranspilationDependency::Registry(RustTranspilationRegistryDep { package_path })
            }
            PackagePathSource::Local { .. } => {
                RustTranspilationDependency::Local(RustTranspilationLocalDep { package_path })
            }
            PackagePathSource::Git { .. } => todo!(),
        }
    }

    pub(crate) fn transpile_to_fs(&self, db: &dyn RustTranspilationDb) {
        match self {
            RustTranspilationDependency::Library(slf) => slf.transpile_to_fs(db),
            RustTranspilationDependency::Registry(slf) => slf.transpile_to_fs(db),
            RustTranspilationDependency::Local(slf) => slf.transpile_to_fs(db),
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

pub(crate) fn rust_transpilation_dependencies(
    target_path: LinktimeTargetPath,
    db: &dyn RustTranspilationDb,
) -> &[RustTranspilationDependency] {
    match target_path {
        LinktimeTargetPath::Package(package_path) => {
            package_rust_transpilation_dependencies(db, package_path)
        }
        LinktimeTargetPath::Workspace(workspace_path) => {
            workspace_rust_transpilation_dependencies(db, workspace_path)
        }
    }
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_rust_transpilation_dependencies(
    db: &dyn RustTranspilationDb,
    package_path: PackagePath,
) -> Vec<RustTranspilationDependency> {
    let package_deps = package_path
        .package_deps(db)
        .expect("no error at this stage");
    package_deps
        .iter()
        .map(|dep| RustTranspilationDependency::from_manifest(dep, db))
        .collect()
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn workspace_rust_transpilation_dependencies(
    db: &dyn RustTranspilationDb,
    workspace_path: WorkspacePath,
) -> Vec<RustTranspilationDependency> {
    todo!()
}
