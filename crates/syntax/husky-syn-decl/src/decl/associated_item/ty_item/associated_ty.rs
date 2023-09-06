use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub node: TypeItemSynNode,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        Default::default()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub syn_expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
