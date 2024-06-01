#[salsa::jar]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::package::module_relative_path_for_transpilation,
    crate::linkage::package_linkages_transpilation,
    crate::manifest::linktime_target_rust_workspace_manifest,
    crate::manifest::package_source_rust_package_manifest,
    crate::manifest::package_linkages_rust_package_manifest,
);
