use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantDecl {
    #[id]
    pub path: VariantPath,
    pub expr_region: ExprRegion,
}
