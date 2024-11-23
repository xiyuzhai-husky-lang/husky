#[salsa::jar]
pub struct LnEntityPathJar(
    crate::menu::ln_item_path_menu,
    crate::namespace::LnNamespace,
    crate::namespace::ln_namespace_all_idents,
);
