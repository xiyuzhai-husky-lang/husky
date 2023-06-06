use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

impl TraitForTypeMethodFnDecl {
    pub fn self_parameter<'a>(self, db: &'a dyn DeclDb) -> Option<&'a SelfParameterDeclPattern> {
        self.explicit_parameter_decl_list(db)
            .self_parameter()
            .as_ref()
    }

    pub fn regular_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.explicit_parameter_decl_list(db).regular_parameters()
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_trai_for_ty_method_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitForTypeMethodFnDecl> {
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
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TraitForTypeItem(path)) => path,
            None => todo!(),
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectedParameterDeclList)?;

        let curry_token = ctx.parse()?;
        let return_ty = if curry_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectedOutputType)?)
        } else {
            None
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        Ok(TraitForTypeMethodFnDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        ))
    }
}
