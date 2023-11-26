use husky_io_utils::error::IOResult;
use husky_manifest::HasPackageManifest;
use husky_vfs::{
    path::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData},
    PackagePathSource,
};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) enum RustTranspilationPackage {
    Library(RustTranspilationLibraryPackage),
    Registry(RustTranspilationRegistryPackage),
    Local(RustTranspilationLocalPackage),
}

impl RustTranspilationPackage {
    fn new(package_path: PackagePath, target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        match package_path.data(db) {
            PackagePathSource::Library => {
                RustTranspilationPackage::Library(RustTranspilationLibraryPackage { package_path })
            }
            PackagePathSource::Registry { .. } => {
                RustTranspilationPackage::Registry(RustTranspilationRegistryPackage {
                    package_path,
                })
            }
            PackagePathSource::Local { .. } => {
                RustTranspilationPackage::Local(RustTranspilationLocalPackage {
                    target_path,
                    package_path,
                })
            }
            PackagePathSource::Git { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationLibraryPackage {
    package_path: PackagePath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationRegistryPackage {
    package_path: PackagePath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationLocalPackage {
    target_path: LinktimeTargetPath,
    package_path: PackagePath,
}

impl RustTranspilationLocalPackage {
    pub(crate) fn target_path(&self) -> LinktimeTargetPath {
        self.target_path
    }

    pub(crate) fn package_path(&self) -> PackagePath {
        self.package_path
    }
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn rust_transpilation_packages(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> Vec<RustTranspilationPackage> {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => {
            let mut packages = vec![RustTranspilationPackage::new(package_path, target_path, db)];
            packages.extend(
                package_path
                    .package_deps(db)
                    .expect("no error at this stage")
                    .iter()
                    .map(|dep| RustTranspilationPackage::new(dep.package_path(), target_path, db)),
            );
            packages
        }
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}
