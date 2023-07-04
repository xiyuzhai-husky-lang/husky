use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeNodeDecl {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedTypeNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_for_ty_associated_ty_node_decl(
        &self,
        node_path: TraitForTypeItemNodePath,
        node: TraitForTypeItemNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitForTypeAssociatedTypeNodeDecl {
        let db = self.db();
        let impl_block_node_decl = node_path.impl_block(db).node_decl(db);
        let mut parser = self.expr_parser(
            node.node_path(db),
            Some(impl_block_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        TraitForTypeAssociatedTypeNodeDecl::new(
            db,
            node.node_path(db),
            node,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedTypeDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeItemPath,
        node_decl: TraitForTypeAssociatedTypeNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(TraitForTypeAssociatedTypeDecl::new(
            db,
            path,
            implicit_parameters,
            expr_region,
        ))
    }
}
