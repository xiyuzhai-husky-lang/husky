use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynNodeDecl {
    #[id]
    pub id: TypeItemSynNodePath,
    pub node: TypeItemSynNode,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeItemPath,
        syn_node_decl: TypeAssociatedValSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
