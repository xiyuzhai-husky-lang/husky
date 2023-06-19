use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeNodeDecl {
    #[id]
    pub node_id: TraitForTypeItemNodeId,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeDecl {
    #[id]
    pub node_id: TraitForTypeItemNodeId,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_trai_for_ty_associated_ty_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TraitForTypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitForTypeAssociatedTypeDecl> {
        let db = self.db();
        let Ok(impl_decl) = node.impl_block(db).decl(db) else {
            return Err(
                DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodFnDecl.into()
            )
        };
        let mut parser = self.expr_parser(
            node.node_id(db),
            Some(impl_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        // let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        Ok(TraitForTypeAssociatedTypeDecl::new(
            db,
            node.node_id(db),
            node,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        ))
    }
}
