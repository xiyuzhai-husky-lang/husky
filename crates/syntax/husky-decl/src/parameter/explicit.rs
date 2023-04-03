use parsec::parse_separated_list2;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[derive(Getters)]
pub struct ExplicitParameterDeclList {
    lpar: LeftParenthesisToken,
    self_parameter: Option<SelfParameterDeclPattern>,
    regular_parameters: Vec<RegularParameterDeclPattern>,
    commas: Vec<CommaToken>,
    rpar: RightParenthesisToken,
}

impl<'a, 'b> ParseFromStreamWithError<ExprParseContext<'a, 'b>> for ExplicitParameterDeclList {
    type Error = DeclExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, DeclExprError> {
        let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(None)
        };
        let self_parameter: Option<SelfParameterDeclPattern> = ctx.parse()?;
        if self_parameter.is_some() {
            todo!()
        }
        let (regular_parameters, commas) = parse_separated_list2(ctx, |e| e)?;
        let rpar =
            ctx.parse_expected(OriginalDeclExprError::ExpectRightParenthesisInParameterList)?;
        Ok(Some(Self {
            lpar,
            self_parameter,
            regular_parameters,
            commas,
            rpar,
        }))
    }
}
