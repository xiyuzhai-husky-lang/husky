use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoizedFieldDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub memo_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr_or_eol_token: Either<EolToken, ExprIdx>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeMemoizedFieldDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(
            self.db()
        ) else { todo!() };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };

        let colon_token = ctx.parse()?;
        let form_ty = if colon_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectOutputType)?)
        } else {
            None
        };
        let eq_token = ctx.parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable)?;
        let expr_or_eol_token = if let Some(eol_token) = ctx.parse::<EolToken>()? {
            Left(eol_token)
        } else {
            Right(todo!("parse expr"))
        };
        Ok(TypeMemoizedFieldDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr_or_eol_token,
            parser.finish(),
        )
        .into())
    }
}
