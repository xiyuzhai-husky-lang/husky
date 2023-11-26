use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct IllFormedImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedImplBlockSynNodePath,
    pub syn_expr_region: SynExprRegion,
    // ad hoc
}

impl IllFormedImplBlockSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // ad hoc
        SmallVec::default()
    }
}

impl HasSynNodeDecl for IllFormedImplBlockSynNodePath {
    type NodeDecl = IllFormedImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        ill_formed_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ill_formed_impl_block_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> IllFormedImplBlockSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path);
    parser.parse_ill_formed_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a, IllFormedImplBlockSynNodePath> {
    fn parse_ill_formed_impl_block_syn_node_decl(
        &self,
        _syn_node_path: IllFormedImplBlockSynNodePath,
    ) -> IllFormedImplBlockSynNodeDecl {
        let db = self.db();
        let parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        IllFormedImplBlockSynNodeDecl::new(db, self.syn_node_path(), parser.finish())
    }
}
