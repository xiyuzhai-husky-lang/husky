use crate::{
    defn::module_defn_rust_transpilation,
    linkage::package_linkages_transpilation,
    manifest::{
        linktime_target_rust_workspace_manifest, package_linkages_rust_package_manifest,
        package_source_rust_package_manifest,
    },
    package::{rust_transpilation_packages, RustTranspilationPackage},
    *,
};
use husky_entity_syn_tree::helpers::paths::crate_module_paths;
use husky_io_utils::error::IOResult;
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use is::Is;

pub trait TranspileToFsFull: Is<LinktimeTargetPath> {
    /// transpile the target crate and its dependencies
    fn transpile_to_fs_full(self, db: &::salsa::Db) -> IOResult<()>;
}

impl TranspileToFsFull for LinktimeTargetPath {
    fn transpile_to_fs_full(self, db: &::salsa::Db) -> IOResult<()> {
        husky_io_utils::diff_write(
            self.rust_workspace_manifest_path(db),
            linktime_target_rust_workspace_manifest(db, self),
            true,
        );
        for package in rust_transpilation_packages(db, self) {
            package.transpile_to_fs(db)?
        }
        Ok(())
    }
}

impl RustTranspilationPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        let workspace_dir = self.target_path.rust_workspace_dir(db);
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
