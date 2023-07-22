use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValNodeDecl {
    #[id]
    pub id: TypeItemSynNodePath,
    pub node: TypeItemSynNode,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedValNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValDecl {
    #[id]
    pub path: TypeItemPath,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedValDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeItemPath,
        syn_node_decl: TypeAssociatedValNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
