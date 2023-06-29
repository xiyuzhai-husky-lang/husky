use super::*;

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitTypeVariantNodeDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitTypeVariantDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}
