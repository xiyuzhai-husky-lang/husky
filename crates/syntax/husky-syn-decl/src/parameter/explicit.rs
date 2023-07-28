use super::*;
use parsec::{parse_separated_list2, parse_separated_small_list2};

pub(crate) type ExplicitParameterDeclPatterns = SmallVec<[SpecificParameterDecl; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[derive(Getters)]
pub struct SelfParameterAndExplicitParameters<const ALLOW_SELF_PARAMETER: bool> {
    lpar: LeftParenthesisToken,
    self_parameter: Option<SelfParameterDeclPattern>,
    comma_after_self_parameter: Option<CommaToken>,
    parenic_parameters: ExplicitParameterDeclPatterns,
    commas: CommaTokens,
    rpar: RightParenthesisToken,
}

impl<'a, 'b, const ALLOW_SELF_PARAMETER: bool> TryParseOptionFromStream<ExprParseContext<'a, 'b>>
    for SelfParameterAndExplicitParameters<ALLOW_SELF_PARAMETER>
{
    type Error = NodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, NodeDeclError> {
        let Some(lpar) = ctx.try_parse_option::<LeftParenthesisToken>()? else {
            return Ok(None);
        };
        let self_parameter: Option<SelfParameterDeclPattern> = ctx.try_parse_option()?;
        let comma_after_self_parameter = if self_parameter.is_some() {
            ctx.try_parse_err_as_none::<CommaToken>()
        } else {
            None
        };
        let (parenic_parameters, commas) =
            if self_parameter.is_none() || comma_after_self_parameter.is_some() {
                parse_separated_small_list2(ctx, |e| e)?
            } else {
                Default::default()
            };
        let rpar =
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedRightParenthesisInParameterList)?;
        Ok(Some(Self {
            lpar,
            self_parameter,
            comma_after_self_parameter,
            parenic_parameters,
            commas,
            rpar,
        }))
    }
}
