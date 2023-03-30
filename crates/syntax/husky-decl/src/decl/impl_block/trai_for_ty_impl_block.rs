use husky_print_utils::p;
use salsa::DebugWithDb;

use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeImplBlockDecl {
    pub ast_idx: AstIdx,
    pub impl_block: TraitForTypeImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub trai_expr: TraitExpr,
    #[return_ref]
    pub for_token: ConnectionForToken,
    pub ty_expr: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeImplBlockDecl {
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

pub(super) fn trai_for_ty_impl_block_decl(
    db: &dyn DeclDb,
    impl_block: TraitForTypeImplBlock,
) -> DeclResultRef<TraitForTypeImplBlockDecl> {
    trai_for_ty_impl_block_decl_aux(db, impl_block)
        .as_ref()
        .copied()
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn trai_for_ty_impl_block_decl_aux(
    db: &dyn DeclDb,
    impl_block: TraitForTypeImplBlock,
) -> DeclResult<TraitForTypeImplBlockDecl> {
    let parser = DeclParseContext::new(db, impl_block.module_path(db))?;
    Ok(parser.parse_trai_for_ty_impl_block_decl(impl_block)?.into())
}

impl HasDecl for TraitForTypeImplBlock {
    type Decl = TraitForTypeImplBlockDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        trai_for_ty_impl_block_decl(db, self)
    }
}

impl<'a> DeclParseContext<'a> {
    fn parse_trai_for_ty_impl_block_decl(
        &self,
        impl_block: TraitForTypeImplBlock,
    ) -> DeclResult<TraitForTypeImplBlockDecl> {
        let ast_idx = impl_block.ast_idx(self.db());
        match self.ast_sheet()[ast_idx] {
            Ast::Impl {
                token_group_idx,
                body: _,
            } => Ok(self
                .parse_trai_for_ty_impl_block_decl_aux(ast_idx, token_group_idx, impl_block)?
                .into()),
            _ => unreachable!(),
        }
    }

    fn parse_trai_for_ty_impl_block_decl_aux(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        impl_block: TraitForTypeImplBlock,
    ) -> DeclResult<TraitForTypeImplBlockDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::ImplBlock(impl_block.id(self.db()).into()),
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
        // ad hoc
        let trai: TraitExpr = ctx.parse().unwrap().unwrap();
        let for_token = ctx.parse().unwrap().unwrap();
        let ty = ctx.parse().unwrap().unwrap();
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TraitForTypeImplBlockDecl::new(
            self.db(),
            ast_idx,
            impl_block,
            impl_token,
            implicit_parameter_decl_list,
            trai,
            for_token,
            ty,
            eol_colon,
            parser.finish(),
        ))
    }
}
