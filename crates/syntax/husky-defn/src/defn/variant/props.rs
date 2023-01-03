use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: PropsVariantDecl,
    pub expr_sheet: ExprSheet,
}
