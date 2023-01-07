use super::*;
use husky_expr::ExprIdx;
use husky_token::{
    ColonToken, CommaToken, IdentifierToken, LeftCurlyBraceToken, RightCurlyBraceToken,
};
use husky_word::Identifier;
use parsec::ParseContext;

#[salsa::tracked(jar = DeclJar)]
pub struct PropsStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_sheet: ModuleItemDeclExprSheet,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub fields: Vec<PropsStructFieldDecl>,
    #[return_ref]
    pub separators: Vec<CommaToken>,
    pub rcurl: RightCurlyBraceToken,
}

impl PropsStructTypeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropsStructFieldDecl {
    ident: IdentifierToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl<'a, 'b, S> parsec::ParseFrom<ExprParseContext<'a, 'b, S>> for PropsStructFieldDecl
where
    S: SymbolContextMut,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b, S>,
    ) -> Result<Option<Self>, ExprError> {
        let ident: IdentifierToken = ctx.parse_expected()?;
        let colon: ColonToken = ctx.parse_expected()?;
        let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) else { todo!() };
        Ok(Some(PropsStructFieldDecl {
            ident,
            colon,
            ty: expr,
        }))
    }
}
