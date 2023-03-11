use super::*;
use husky_token::EolColonToken;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplDecl {
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub ty: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_impl_decl(db: &dyn DeclDb, impl_block: TypeImplBlock) -> DeclResult<TypeImplDecl> {
    let parser = DeclParser::new(db, impl_block.module_path(db))?;
    Ok(parser.parse_ty_impl_decl(impl_block)?.into())
}

impl TypeImplDecl {
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
}

impl<'a> DeclParser<'a> {
    fn parse_ty_impl_decl(&self, impl_block: TypeImplBlock) -> DeclResult<TypeImplDecl> {
        let ast_idx = impl_block.ast_idx(self.db());
        match self.ast_sheet()[ast_idx] {
            Ast::Impl {
                token_group_idx,
                body: _,
            } => Ok(self
                .parse_ty_impl_decl_aux(ast_idx, token_group_idx, impl_block)?
                .into()),
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_decl_aux(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        impl_block: TypeImplBlock,
    ) -> DeclResult<TypeImplDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Impl(impl_block.id(self.db()).into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data()
                .token_group_token_stream(token_group_idx, None),
        );
        let impl_token = ctx.parse().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.parse();
        let ty = ctx.parse().unwrap().unwrap();
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeImplDecl::new(
            self.db(),
            ast_idx,
            impl_block,
            impl_token,
            implicit_parameter_decl_list,
            ty,
            eol_colon,
            parser.finish(),
        ))
    }
}
