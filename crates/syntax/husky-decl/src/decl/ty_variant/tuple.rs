use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantNodeDecl {
    #[id]
    pub node_id: TypeVariantNodeId,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleVariantDecl {
    #[id]
    pub id: TypeVariantNodeId,
    pub expr_region: ExprRegion,
}
