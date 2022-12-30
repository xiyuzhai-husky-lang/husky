use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct PropsVariantDecl {
    #[id]
    pub path: VariantPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
