use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantDecl {
    #[id]
    pub path: VariantPath,
    pub expr_region: ExprRegion,
}
