use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct PropsVariantDecl {
    #[id]
    pub path: VariantPath,
    pub expr_sheet: VariantDeclExprSheet,
}
