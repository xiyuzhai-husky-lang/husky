#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::linkage::package_linkages_transpilation,
    crate::manifest::linktime_target_rust_workspace_manifest,
    crate::manifest::package_source_rust_package_manifest,
    crate::manifest::package_linkages_rust_package_manifest,
);
