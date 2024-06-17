#[salsa::jar]
pub struct InlayHintsJar(
    crate::inlay_hints::crate_decl::crate_decl_inlay_hints,
    crate::inlay_hints::item_decl::item_decl_inlay_hints,
    crate::inlay_hints::item_defn::item_defn_inlay_hints,
);
