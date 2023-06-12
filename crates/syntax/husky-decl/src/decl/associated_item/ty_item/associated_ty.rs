use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedTypeNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedTypeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
