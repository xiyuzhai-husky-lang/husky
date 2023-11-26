#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::linkages::package_linkages_transpilation,
    crate::manifest::package_rust_manifest,
);
