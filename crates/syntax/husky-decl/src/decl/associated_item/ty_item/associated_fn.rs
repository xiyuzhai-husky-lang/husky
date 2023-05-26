use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedFnDecl {
    #[id]
    pub id: AssociatedItemId,
    pub path: Option<TypeItemPath>,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl TypeAssociatedFnDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.parameter_decl_list(db).regular_parameters()
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_associated_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeAssociatedFnDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(self.db())
        else { return Err(DerivedDeclError::UnableToParseImplDeclForTyMethodFnDecl.into()) };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
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
        Ok(TypeAssociatedFnDecl::new(
            self.db(),
            associated_item.id(self.db()),
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        ))
    }
}
