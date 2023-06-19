use parsec::parse_separated_list2;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[derive(Getters)]
pub struct ExplicitParameterDeclList {
    lpar: LeftParenthesisToken,
    self_parameter: Option<SelfParameterDeclPattern>,
    comma_after_self_parameter: Option<CommaToken>,
    regular_parameters: Vec<RegularParameterDeclPattern>,
    commas: Vec<CommaToken>,
    rpar: RightParenthesisToken,
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for ExplicitParameterDeclList {
    type Error = DeclExprError;

    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, DeclExprError> {
        let Some(lpar) = ctx.try_parse_optional::<LeftParenthesisToken>()? else {
            return Ok(None)
        };
        let self_parameter: Option<SelfParameterDeclPattern> = ctx.try_parse_optional()?;
        let comma_after_self_parameter = if self_parameter.is_some() {
            ctx.parse_err_as_none::<CommaToken>()
        } else {
            None
        };
        let (regular_parameters, commas) =
            if self_parameter.is_none() || comma_after_self_parameter.is_some() {
                parse_separated_list2(ctx, |e| e)?
            } else {
                Default::default()
            };
        let rpar =
            ctx.parse_expected(OriginalDeclExprError::ExpectedRightParenthesisInParameterList)?;
        Ok(Some(Self {
            lpar,
            self_parameter,
            comma_after_self_parameter,
            regular_parameters,
            commas,
            rpar,
        }))
    }
}
