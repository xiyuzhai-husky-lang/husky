use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: UnitVariantDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
