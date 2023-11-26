use crate::{
    defn::module_defn_rust_transpilation,
    linkages::package_linkages_transpilation,
    manifest::package_rust_manifest,
    package::{
        rust_transpilation_packages, RustTranspilationLibraryPackage,
        RustTranspilationLocalPackage, RustTranspilationPackage, RustTranspilationRegistryPackage,
    },
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
        for dep in rust_transpilation_packages(db, self) {
            dep.transpile_to_fs(db)?
        }
        Ok(())
    }
}

impl RustTranspilationPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        match self {
            RustTranspilationPackage::Library(slf) => slf.transpile_to_fs(db),
            RustTranspilationPackage::Registry(slf) => slf.transpile_to_fs(db),
            RustTranspilationPackage::Local(slf) => slf.transpile_to_fs(db),
        }
    }
}

impl RustTranspilationLibraryPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        // ad hoc
        Ok(())
    }
}

impl RustTranspilationRegistryPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        // ad hoc
        Ok(())
    }
}

impl RustTranspilationLocalPackage {
    pub(crate) fn transpile_to_fs(&self, db: &::salsa::Db) -> IOResult<()> {
        let rust_dir = self.target_path().rust_dir(db);
        transpile_package_to_fs(rust_dir, self.package_path(), db)
    }
}

fn transpile_package_to_fs(
    rust_dir: &std::path::Path,
    package_path: PackagePath,
    db: &::salsa::Db,
) -> IOResult<()> {
    let package_dir = rust_dir.join(package_path.name(db).data(db));
    let src_dir = package_dir.join("src");
    let cargo_toml_path = package_dir.join("Cargo.toml");
    husky_io_utils::diff_write(
        &cargo_toml_path,
        package_rust_manifest(db, package_path),
        true,
    );
    husky_io_utils::diff_write(
        &src_dir.join("__linkages.rs"),
        package_linkages_transpilation(db, package_path),
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
