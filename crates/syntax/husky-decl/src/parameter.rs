use husky_opn_syntax::Bracket;
use husky_print_utils::p;
use husky_token::*;
use parsec::{parse_separated_list, ParseContext, ParseFrom};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterDecl {
    pattern: ImplicitParameterDeclPattern,
    traits: Option<(ColonToken, Option<ExprIdx>)>,
}

impl ImplicitParameterDecl {
    pub fn pattern(&self) -> &ImplicitParameterDeclPattern {
        &self.pattern
    }

    pub fn traits(&self) -> Option<(ColonToken, Option<ExprIdx>)> {
        self.traits
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, OriginalExprError> {
        let Some(pattern) = ctx.parse::<ImplicitParameterDeclPattern>()? else {
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
pub struct ImplicitParameterDeclList {
    langle: LeftAngleBracketOrLessThanToken,
    implicit_parameters: Vec<ImplicitParameterDecl>,
    commas: Vec<CommaToken>,
    rangle: RightAngleBracketToken,
}

impl std::ops::Deref for ImplicitParameterDeclList {
    type Target = Vec<ImplicitParameterDecl>;

    fn deref(&self) -> &Self::Target {
        &self.implicit_parameters
    }
}

impl ImplicitParameterDeclList {
    pub fn lcurl(&self) -> LeftAngleBracketOrLessThanToken {
        self.langle
    }

    pub fn implicit_parameters(&self) -> &[ImplicitParameterDecl] {
        self.implicit_parameters.as_ref()
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }

    pub fn rcurl(&self) -> RightAngleBracketToken {
        self.rangle
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDeclList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, OriginalExprError> {
        let Some(langle) = ctx.parse::< LeftAngleBracketOrLessThanToken>()? else {
            return Ok(None)
        };
        let (decls, commas) = parse_separated_list(ctx)?;
        Ok(Some(Self {
            langle,
            implicit_parameters: decls,
            commas,
            rangle: ctx
                .parse()?
                .ok_or(OriginalExprError::MissingRightAngleBracket {
                    langle_token_idx: langle.token_idx(),
                })?,
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterDeclList {
    lpar: LeftParenthesisToken,
    parameters: Vec<RegularParameterDeclPattern>,
    commas: Vec<CommaToken>,
    rpar: RightParenthesisToken,
}

impl ParameterDeclList {
    pub fn parameters(&self) -> &[RegularParameterDeclPattern] {
        self.parameters.as_ref()
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ParameterDeclList {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, OriginalExprError> {
        let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(None)
        };
        let (decls, commas) = parse_separated_list(ctx)?;
        let rpar = ctx.parse_expected::<RightParenthesisToken>()?;
        Ok(Some(Self {
            lpar,
            parameters: decls,
            commas,
            rpar,
        }))
    }
}
