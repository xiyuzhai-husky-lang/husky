use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a, TypeItemSynNodePath> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynDecl {
    pub(super) fn from_node_decl(
        _db: &::salsa::Db,
        _path: TypeItemPath,
        _syn_node_decl: TypeAssociatedValSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
