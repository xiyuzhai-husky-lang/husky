use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantNodeDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantDecl {
    #[id]
    pub id: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}
