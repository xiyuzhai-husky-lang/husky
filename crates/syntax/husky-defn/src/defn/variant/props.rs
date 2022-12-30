use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: PropsVariantDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
