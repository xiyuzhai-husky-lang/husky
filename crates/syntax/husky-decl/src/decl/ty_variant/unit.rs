use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantNodeDecl {
    #[id]
    pub node_id: TypeVariantNodeId,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitVariantDecl {
    #[id]
    pub node_id: TypeVariantNodeId,
    pub expr_region: ExprRegion,
}
