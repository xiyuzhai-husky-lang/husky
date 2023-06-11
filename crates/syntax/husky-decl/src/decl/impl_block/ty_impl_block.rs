use super::*;
use husky_token::EolToken;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplBlockRawDecl {
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub ty_expr: TypeExpr,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplBlockDecl {
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub ty_expr: TypeExpr,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

impl TypeImplBlockDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

impl HasDecl for TypeImplBlock {
    type Decl = TypeImplBlockDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        ty_impl_block_decl_aux(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_impl_block_decl_aux(
    db: &dyn DeclDb,
    impl_block: TypeImplBlock,
) -> DeclResult<TypeImplBlockDecl> {
    let parser = DeclParseContext::new(db, impl_block.module_path(db))?;
    Ok(parser.parse_ty_impl_block_decl(impl_block)?.into())
}

impl<'a> DeclParseContext<'a> {
    fn parse_ty_impl_block_decl(&self, impl_block: TypeImplBlock) -> DeclResult<TypeImplBlockDecl> {
        let ast_idx = impl_block.ast_idx(self.db());
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => Ok(self
                .parse_ty_impl_block_decl_aux(ast_idx, token_group_idx, impl_block)?
                .into()),
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_block_decl_aux(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        impl_block: TypeImplBlock,
    ) -> DeclResult<TypeImplBlockDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::ImplBlock(impl_block.id(self.db()).into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.parse().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.parse()?;
        let ty = ctx.parse().unwrap().unwrap();
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        Ok(TypeImplBlockDecl::new(
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
