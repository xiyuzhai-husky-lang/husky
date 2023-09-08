use super::*;
use parsec::{parse_separated_list2, parse_separated_small_list2};

pub(crate) type ExplicitParameterDeclPatterns = SmallVec<[SpecificParameterObelisk; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[derive(Getters)]
pub struct RitchieParameters<const ALLOW_SELF_PARAMETER: bool> {
    lpar: LparToken,
    self_value_parameter: Option<SelfParameterObelisk>,
    comma_after_self_parameter: Option<CommaToken>,
    parenate_parameters: ExplicitParameterDeclPatterns,
    commas: CommaTokens,
    rpar: RparToken,
}

impl<'a, const ALLOW_SELF_PARAMETER: bool> TryParseOptionFromStream<SynDeclExprParser<'a>>
    for RitchieParameters<ALLOW_SELF_PARAMETER>
{
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, SynNodeDeclError> {
        let Some(lpar) = ctx.try_parse_option::<LparToken>()? else {
            return Ok(None);
        };
        let self_value_parameter: Option<SelfParameterObelisk> = ctx.try_parse_option()?;
        let comma_after_self_parameter = if self_value_parameter.is_some() {
            ctx.try_parse_err_as_none::<CommaToken>()
        } else {
            None
        };
        let (parenate_parameters, commas) =
            if self_value_parameter.is_none() || comma_after_self_parameter.is_some() {
                parse_separated_small_list2(ctx, |e| e)?
            } else {
                Default::default()
            };
        let rpar = ctx.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList,
        )?;
        Ok(Some(Self {
            lpar,
            self_value_parameter,
            comma_after_self_parameter,
            parenate_parameters,
            commas,
            rpar,
        }))
    }
}
