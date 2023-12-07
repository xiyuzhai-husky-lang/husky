use std::path::Path;

use husky_entity_syn_tree::helpers::paths::crate_module_paths;
use husky_io_utils::error::IOResult;
use husky_manifest::HasPackageManifest;
use husky_print_utils::p;
use husky_vfs::{
    path::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData},
    PackagePathSource,
};
use pathdiff::diff_paths;

use crate::{
    defn::module_defn_rust_transpilation,
    linkage::package_linkages_transpilation,
    manifest::{package_linkages_rust_package_manifest, package_source_rust_package_manifest},
    *,
};

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl RustTranspilationPackage {
    pub(crate) fn name(self, db: &::salsa::Db) -> String {
        match self.kind {
            RustTranspilationPackageKind::Source => self.package_path.name(db).data(db).to_string(),
            RustTranspilationPackageKind::Linkages => {
                format!("{}-linkages", self.package_path.name(db).data(db))
            }
        }
    }

    pub(crate) fn path_in_workspace(
        self,
        rust_workspace_abs_dir: &Path,
        db: &::salsa::Db,
    ) -> String {
        use salsa::DebugWithDb;
        if self.package_path.is_virtual(db) {
            diff_paths(
                self.package_path.dir(db).unwrap().abs_path(db).unwrap(),
                rust_workspace_abs_dir,
            )
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
        } else {
            match self.kind {
                RustTranspilationPackageKind::Source => {
                    self.package_path.name(db).data(db).to_string()
                }
                RustTranspilationPackageKind::Linkages => {
                    format!("{}/linkages", self.package_path.name(db).data(db))
                }
            }
        }
    }
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
                                package_path: dep.package_path(),
                                kind: RustTranspilationPackageKind::Source,
                            },
                            RustTranspilationPackage {
                                target_path,
                                package_path: dep.package_path(),
                                kind: RustTranspilationPackageKind::Linkages,
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

#[test]
fn rust_transpilation_packages_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, package_path: PackagePath| {
            let linktime_target_path = LinktimeTargetPath::new_package(package_path, db);
            rust_transpilation_packages(db, linktime_target_path)
        },
        &AstTestConfig::new("rust_transpilation_packages"),
    )
}

impl RustTranspilationPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        let workspace_dir = self.target_path.rust_workspace_abs_dir(db);
        match self.kind {
            package::RustTranspilationPackageKind::Source => {
                transpile_package_source_to_fs(workspace_dir, self.package_path, db)
            }
            package::RustTranspilationPackageKind::Linkages => {
                transpile_package_linkages_to_fs(workspace_dir, self.package_path, db)
            }
        }
    }
}

fn transpile_package_source_to_fs(
    rust_workspace_dir: &std::path::Path,
    package_path: PackagePath,
    db: &::salsa::Db,
) -> IOResult<()> {
    if package_path.is_virtual(db) {
        return Ok(());
    }
    let package_dir = rust_workspace_dir.join(package_path.name(db).data(db));
    let src_dir = package_dir.join("src");
    let cargo_toml_path = package_dir.join("Cargo.toml");
    husky_io_utils::diff_write(
        &cargo_toml_path,
        package_source_rust_package_manifest(db, package_path),
        true,
    );
    for &crate_path in package_path.crate_paths(db) {
        for &module_path in crate_module_paths(db, crate_path) {
            husky_io_utils::diff_write(
                &module_path.relative_path(db).to_path(&src_dir),
                module_defn_rust_transpilation(db, module_path),
                true,
            );
        }
    }
    Ok(())
}

fn transpile_package_linkages_to_fs(
    rust_workspace_dir: &std::path::Path,
    package_path: PackagePath,
    db: &::salsa::Db,
) -> IOResult<()> {
    let package_dir = rust_workspace_dir
        .join(package_path.name(db).data(db))
        .join("linkages");
    let src_dir = package_dir.join("src");
    let cargo_toml_path = package_dir.join("Cargo.toml");
    husky_io_utils::diff_write(
        &cargo_toml_path,
        package_linkages_rust_package_manifest(db, package_path),
        true,
    );
    husky_io_utils::diff_write(
        &src_dir.join("lib.rs"),
        package_linkages_transpilation(db, package_path),
        true,
    );
    Ok(())
}
