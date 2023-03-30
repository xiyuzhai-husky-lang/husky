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
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    parameter_decl_list: DeclExprResult<ExplicitParameterDeclList>,
    #[return_ref]
    pub curry_token: DeclExprResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclExprResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
}

impl TypeAssociatedFnDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        self.implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(Ok(&[]))
    }

    pub fn parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ExplicitParameterDeclPattern]> {
        self.parameter_decl_list(db).as_ref()?.parameters()
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_associated_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeAssociatedFnDecl> {
        let Ok(impl_decl) = self.db().impl_block_decl(associated_item.impl_block(self.db()))
        else { return Err(DerivedDeclError::UnableToParseImplDeclForTyMethodDecl.into()) };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse();
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
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
