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
    pub generic_parameters: Vec<GenericParameter>,
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
        let (exprs, stop_reason) = ctx.parse_exprs();
        match stop_reason {
            ExprParsingStopReason::Semicolon => todo!(),
            ExprParsingStopReason::NoTokens => todo!(),
            ExprParsingStopReason::Comma => (),
        }
        if exprs.end() - exprs.start() != 1 {
            todo!()
        }
        Ok(Some(PropsStructFieldDecl {
            ident,
            colon,
            ty: exprs.start(),
        }))
    }
}
