use husky_opn_syntax::Bracket;
use husky_print_utils::p;
use husky_token::*;
use parsec::{parse_separated_list, ParseContext, ParseFrom};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignature {
    pattern: ImplicitParameterSignaturePattern,
    traits: Option<(ColonToken, Option<ExprIdx>)>,
}

impl ImplicitParameterSignature {
    pub fn pattern(&self) -> &ImplicitParameterSignaturePattern {
        &self.pattern
    }

    pub fn traits(&self) -> Option<(ColonToken, Option<ExprIdx>)> {
        self.traits
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterSignature {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(pattern) = ctx.parse::<ImplicitParameterSignaturePattern>()? else {
            return Ok(None)
        };

        Ok(Some(Self {
            pattern,
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

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignatureList {
    langle: LeftAngleBracketOrLessThanToken,
    decls: Vec<ImplicitParameterSignature>,
    commas: Vec<CommaToken>,
    rangle: RightAngleBracketToken,
}

impl std::ops::Deref for ImplicitParameterSignatureList {
    type Target = Vec<ImplicitParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.decls
    }
}

impl ImplicitParameterSignatureList {
    pub fn lcurl(&self) -> LeftAngleBracketOrLessThanToken {
        self.langle
    }

    pub fn decls(&self) -> &[ImplicitParameterSignature] {
        self.decls.as_ref()
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }

    pub fn rcurl(&self) -> RightAngleBracketToken {
        self.rangle
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterSignatureList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(langle) = ctx.parse::< LeftAngleBracketOrLessThanToken>()? else {
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
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    colon: ColonToken,
    ty: ExprIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignatureList {
    lpar: LeftParenthesisToken,
    decls: Vec<ParameterSignature>,
    commas: Vec<CommaToken>,
    rpar: RightParenthesisToken,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ParameterSignature {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(pattern) = ctx.parse::<ParameterSignaturePattern>()? else {
            return Ok(None)
        };
        let state = ctx.state();
        let colon = ctx.parse_expected::<ColonToken>()?;
        let Some(ty) = ctx.parse_expr(ExprParseEnvironment::WithinBracket(Bracket::Par)) else {
            todo!()
        };
        Ok(Some(ParameterSignature { pattern, colon, ty }))
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ParameterSignatureList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(None)
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
