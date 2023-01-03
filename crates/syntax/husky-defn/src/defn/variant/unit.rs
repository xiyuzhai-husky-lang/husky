use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: UnitVariantDecl,
    pub expr_sheet: ExprSheet,
}
