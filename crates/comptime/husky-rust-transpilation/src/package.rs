use husky_io_utils::error::IOResult;
use husky_manifest::HasPackageManifest;
use husky_vfs::{
    path::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData},
    PackagePathSource,
};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = RustTranspilationDb, jar = RustTranspilationJar)]
pub(crate) struct RustTranspilationPackage {
    pub(crate) target_path: LinktimeTargetPath,
    pub(crate) package_path: PackagePath,
    pub(crate) kind: RustTranspilationPackageKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustTranspilationPackageKind {
    Source,
    Linkages,
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn rust_transpilation_packages(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> Vec<RustTranspilationPackage> {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => {
            let mut packages = vec![
                RustTranspilationPackage {
                    target_path,
                    package_path,
                    kind: RustTranspilationPackageKind::Source,
                },
                RustTranspilationPackage {
                    target_path,
                    package_path,
                    kind: RustTranspilationPackageKind::Linkages,
                },
            ];
            packages.extend(
                package_path
                    .package_dependencies(db)
                    .expect("no error at this stage")
                    .iter()
                    .map(|dep| {
                        [
                            RustTranspilationPackage {
                                target_path,
                                package_path,
                                kind: RustTranspilationPackageKind::Source,
                            },
                            RustTranspilationPackage {
                                target_path,
                                package_path,
                                kind: RustTranspilationPackageKind::Source,
                            },
                        ]
                    })
                    .flatten(),
            );
            packages
        }
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}
