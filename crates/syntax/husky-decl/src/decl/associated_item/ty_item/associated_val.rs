use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedValNodeDecl {
    #[id]
    pub id: TypeItemNodeId,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedValDecl {
    #[id]
    pub id: TypeItemNodeId,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
