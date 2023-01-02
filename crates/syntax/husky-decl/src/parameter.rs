use husky_opn_syntax::Bracket;
use husky_print_utils::p;
use husky_token::{
    ColonToken, CommaToken, IdentifierToken, LeftAngleBracketToken, RightAngleBracketToken,
};
use parsec::{parse_separated_list, ParseContext, ParseFrom};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterDecl {
    ident: IdentifierToken,
    traits: Option<(ColonToken, Option<ExprIdx>)>,
}

impl<'a, 'b, 'c> ParseFrom<ExprParser<'a, 'b, 'c>> for ImplicitParameterDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParser<'a, 'b, 'c>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(ident) = ctx.parse::<IdentifierToken>()? else {
            return Ok(None)
        };

        Ok(Some(Self {
            ident,
            traits: if let Some(colon) = ctx.parse::<ColonToken>()? {
                Some((
                    colon,
                    ctx.parse_expr(ExprEnvironment::WithinBracket(Bracket::Angle)),
                ))
            } else {
                None
            },
        }))
    }
}

impl<'a, 'b, 'c> ParseFrom<ExprParser<'a, 'b, 'c>> for ImplicitParameterDeclList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParser<'a, 'b, 'c>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(langle) = ctx.parse::<LeftAngleBracketToken>()? else {
            return Ok(None)
        };
        let (decls, commas) = parse_separated_list(ctx)?;
        Ok(Some(Self {
            langle,
            decls,
            commas,
            rangle: ctx.parse()?.ok_or(ExprError::MissingRightAngleBracket {
                langle_token_idx: langle.token_idx(),
            })?,
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterDeclList {
    langle: LeftAngleBracketToken,
    decls: Vec<ImplicitParameterDecl>,
    commas: Vec<CommaToken>,
    rangle: RightAngleBracketToken,
}

impl ImplicitParameterDeclList {
    pub fn lcurl(&self) -> LeftAngleBracketToken {
        self.langle
    }

    pub fn decls(&self) -> &[ImplicitParameterDecl] {
        self.decls.as_ref()
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }

    pub fn rcurl(&self) -> RightAngleBracketToken {
        self.rangle
    }
}
