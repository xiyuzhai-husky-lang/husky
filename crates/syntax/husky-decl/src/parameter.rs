use husky_opn_syntax::Bracket;
use husky_print_utils::p;
use husky_token::{
    ColonToken, CommaToken, IdentifierToken, LeftAngleBracketToken, LeftParenthesisToken,
    RightAngleBracketToken, RightParenthesisToken,
};
use parsec::{parse_separated_list, ParseContext, ParseFrom};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterDecl {
    ident: IdentifierToken,
    traits: Option<(ColonToken, Option<ExprIdx>)>,
}

impl ImplicitParameterDecl {
    pub fn ident(&self) -> IdentifierToken {
        self.ident
    }

    pub fn traits(&self) -> Option<(ColonToken, Option<ExprIdx>)> {
        self.traits
    }
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(ident) = ctx.parse::<IdentifierToken>()? else {
            return Ok(None)
        };

        Ok(Some(Self {
            ident,
            traits: if let Some(colon) = ctx.parse::<ColonToken>()? {
                Some((
                    colon,
                    ctx.parse_expr(ExprParseEnvironment::WithinBracket(Bracket::Angle)),
                ))
            } else {
                None
            },
        }))
    }
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDeclList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
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

impl std::ops::Deref for ImplicitParameterDeclList {
    type Target = Vec<ImplicitParameterDecl>;

    fn deref(&self) -> &Self::Target {
        &self.decls
    }
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterDecl {
    pattern: ParameterPattern,
    colon: ColonToken,
    ty: ExprIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterDeclList {
    lpar: LeftParenthesisToken,
    decls: Vec<ParameterDecl>,
    commas: Vec<CommaToken>,
    rpar: RightParenthesisToken,
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for ParameterDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(pattern) = ctx.parse::<ParameterPattern >()? else {
            return Ok(None)
        };
        let state = ctx.save_state();
        let colon = ctx.parse_expected::<ColonToken>()?;
        let Some(ty) = ctx.parse_expr(ExprParseEnvironment::WithinBracket(Bracket::Par)) else {
            todo!()
        };
        Ok(Some(ParameterDecl { pattern, colon, ty }))
    }
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for ParameterDeclList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? else {
            todo!()
            // return Ok(None)
        };
        let (decls, commas) = parse_separated_list(ctx)?;
        let rpar = ctx.parse_expected::<RightParenthesisToken>()?;
        Ok(Some(Self {
            lpar,
            decls,
            commas,
            rpar,
        }))
    }
}
