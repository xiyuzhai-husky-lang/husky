use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantRawDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}
