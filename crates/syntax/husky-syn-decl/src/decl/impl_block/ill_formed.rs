use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct IllFormedImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
    // ad hoc
}

impl IllFormedImplBlockSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        SmallVec::default()
    }
}

impl HasSynNodeDecl for IllFormedImplBlockSynNodePath {
    type NodeDecl = IllFormedImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ill_formed_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ill_formed_impl_block_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> IllFormedImplBlockSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_ill_formed_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ill_formed_impl_block_syn_node_decl(
        &self,
        syn_node_path: IllFormedImplBlockSynNodePath,
    ) -> IllFormedImplBlockSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self.parse_ill_formed_impl_block_syn_node_decl_aux(
                syn_node_path,
                node,
                ast_idx,
                token_group_idx,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ill_formed_impl_block_syn_node_decl_aux(
        &self,
        syn_node_path: IllFormedImplBlockSynNodePath,
        node: IllFormedImplBlockSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> IllFormedImplBlockSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            node.syn_node_path(db),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        IllFormedImplBlockSynNodeDecl::new(db, syn_node_path, ast_idx, parser.finish())
    }
}
