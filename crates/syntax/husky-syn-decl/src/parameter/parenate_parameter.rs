use super::*;
use parsec::parse_separated_small_list2;

pub(crate) type ParenateSynParametersData = SmallVec<[ParenateSynParameterData; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[derive(Getters)]
pub struct ParenateParameterSyndicateList<const ALLOW_SELF_PARAMETER: bool> {
    lpar: LparRegionalToken,
    self_value_parameter: Option<SelfValueParameterSyndicate>,
    comma_after_self_parameter: Option<CommaRegionalToken>,
    parenate_parameters: ParenateSynParametersData,
    commas: CommaRegionalTokens,
    rpar: RparRegionalToken,
}

impl<'a, const ALLOW_SELF_PARAMETER: bool> TryParseOptionFromStream<SynDeclExprParser<'a>>
    for ParenateParameterSyndicateList<ALLOW_SELF_PARAMETER>
{
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, SynNodeDeclError> {
        let Some(lpar) = ctx.try_parse_option::<LparRegionalToken>()? else {
            return Ok(None);
        };
        let self_value_parameter: Option<SelfValueParameterSyndicate> = ctx.try_parse_option()?;
        let comma_after_self_parameter = if self_value_parameter.is_some() {
            ctx.try_parse_err_as_none::<CommaRegionalToken>()
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
