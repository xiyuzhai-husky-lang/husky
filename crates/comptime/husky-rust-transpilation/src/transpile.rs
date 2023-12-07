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
