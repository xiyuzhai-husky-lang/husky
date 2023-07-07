use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct IllFormedImplBlockNodeDecl {
    #[id]
    pub node_path: IllFormedImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    // ad hoc
}

impl IllFormedImplBlockNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        SmallVec::default()
    }
}

impl HasNodeDecl for IllFormedImplBlockNodePath {
    type NodeDecl = IllFormedImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ill_formed_impl_block_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ill_formed_impl_block_node_decl(
    db: &dyn DeclDb,
    node_path: IllFormedImplBlockNodePath,
) -> IllFormedImplBlockNodeDecl {
    let parser = DeclParser::new(db, node_path.module_path(db));
    parser.parse_ill_formed_impl_block_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ill_formed_impl_block_node_decl(
        &self,
        node_path: IllFormedImplBlockNodePath,
    ) -> IllFormedImplBlockNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self.parse_ill_formed_impl_block_node_decl_aux(
                node_path,
                node,
                ast_idx,
                token_group_idx,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ill_formed_impl_block_node_decl_aux(
        &self,
        node_path: IllFormedImplBlockNodePath,
        node: IllFormedImplBlockNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> IllFormedImplBlockNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            node.node_path(db),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        IllFormedImplBlockNodeDecl::new(db, node_path, ast_idx, parser.finish())
    }
}
