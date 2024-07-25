#[salsa::jar]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::package::module_relative_path_for_transpilation,
    crate::linket::package_linkets_transpilation,
    crate::manifest::linktime_target_rust_workspace_manifest,
    crate::manifest::source_package_manifest,
    crate::manifest::linkets_package_manifest,
    crate::mangle::item_path_id_interface_cache_path_aux,
);
