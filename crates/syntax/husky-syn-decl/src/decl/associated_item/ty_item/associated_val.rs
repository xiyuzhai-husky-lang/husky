use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParserFactory<'a, TypeItemSynNodePath> {}

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
