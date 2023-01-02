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
    #[return_ref]
    pub implicit_parameters: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub fields: Vec<PropsStructFieldDecl>,
    #[return_ref]
    pub separators: Vec<CommaToken>,
    pub rcurl: RightCurlyBraceToken,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropsStructFieldDecl {
    ident: IdentifierToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl<'a, 'b, 'c> parsec::ParseFrom<ExprParser<'a, 'b, 'c>> for PropsStructFieldDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParser<'a, 'b, 'c>,
    ) -> Result<Option<Self>, ExprError> {
        let ident: IdentifierToken = ctx.parse_expected()?;
        let colon: ColonToken = ctx.parse_expected()?;
        let Some(expr) = ctx.parse_expr(ExprEnvironment::None) else { todo!() };
        Ok(Some(PropsStructFieldDecl {
            ident,
            colon,
            ty: expr,
        }))
    }
}
