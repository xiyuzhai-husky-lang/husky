use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeImplBlockNodeDecl {
    #[id]
    pub node_path: TraitForTypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_block: TraitForTypeImplBlockNode,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub trai_expr: TraitExpr,
    #[return_ref]
    pub for_token: ConnectionForToken,
    pub ty_expr: TypeExpr,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeImplBlockDecl {
    #[id]
    pub node_path: TraitForTypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_block: TraitForTypeImplBlockNode,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub trai_expr: TraitExpr,
    #[return_ref]
    pub for_token: ConnectionForToken,
    pub ty_expr: TypeExpr,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

impl TraitForTypeImplBlockDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

impl HasDecl for TraitForTypeImplBlockPath {
    type Decl = TraitForTypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        todo!()
    }
}

impl HasDecl for TraitForTypeImplBlockNode {
    type Decl = TraitForTypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_impl_block_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trai_for_ty_impl_block_decl(
    db: &dyn DeclDb,
    impl_block: TraitForTypeImplBlockNode,
) -> DeclResult<TraitForTypeImplBlockDecl> {
    let parser = DeclParseContext::new(db, impl_block.module_path(db));
    Ok(parser.parse_trai_for_ty_impl_block_decl(impl_block)?.into())
}

impl<'a> DeclParseContext<'a> {
    fn parse_trai_for_ty_impl_block_decl(
        &self,
        impl_block: TraitForTypeImplBlockNode,
    ) -> DeclResult<TraitForTypeImplBlockDecl> {
        let ast_idx = impl_block.ast_idx(self.db());
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
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
        node: TraitForTypeImplBlockNode,
    ) -> DeclResult<TraitForTypeImplBlockDecl> {
        todo!()
        // let db = self.db();
        // let mut parser = self.expr_parser(
        //     node.node_path(db),
        //     None,
        //     AllowSelfType::True,
        //     AllowSelfValue::False,
        // );
        // let mut ctx = parser.ctx(None, token_group_idx, None);
        // let impl_token = ctx.try_parse_optional().unwrap().unwrap();
        // let implicit_parameter_decl_list = ctx.try_parse_optional()?;
        // // ad hoc
        // let trai: TraitExpr = ctx.try_parse_optional().unwrap().unwrap();
        // let for_token = ctx.try_parse_optional().unwrap().unwrap();
        // let ty = ctx.try_parse_optional().unwrap().unwrap();
        // let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        // Ok(TraitForTypeImplBlockDecl::new(
        //     db,
        //     todo!(),
        //     ast_idx,
        //     node,
        //     impl_token,
        //     implicit_parameter_decl_list,
        //     trai,
        //     for_token,
        //     ty,
        //     eol_colon,
        //     parser.finish(),
        // ))
    }
}
