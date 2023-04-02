use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}
