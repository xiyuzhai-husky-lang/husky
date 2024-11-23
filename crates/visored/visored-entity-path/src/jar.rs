#[salsa::jar]
pub struct VdEntityPathJar(
    crate::module::VdModulePath,
    crate::menu::vd_item_path_menu,
    crate::module::vd_module_lineage,
);
