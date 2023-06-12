use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantRawDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}
