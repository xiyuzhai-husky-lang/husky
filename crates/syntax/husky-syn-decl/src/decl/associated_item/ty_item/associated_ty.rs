use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub node: TypeItemSynNode,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedTypeSynNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        Default::default()
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
