use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeRawDecl {
    pub path: TraitForTypeItemPath,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedTypeDecl {
    pub path: TraitForTypeItemPath,
    pub associated_item: AssociatedItem,
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
        associated_item: AssociatedItem,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitForTypeAssociatedTypeDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(
            self.db()
        ) else {
            return Err(
                DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodFnDecl.into()
            )
        };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TraitForTypeItem(path)) => path,
            None => todo!(),
            _ => unreachable!(),
        };
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TraitForTypeItem(path)) => path,
            None => todo!(),
            _ => unreachable!(),
        };
        // let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        Ok(TraitForTypeAssociatedTypeDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        ))
    }
}
