use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantRawDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}
