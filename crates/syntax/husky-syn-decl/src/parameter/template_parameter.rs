use super::*;
use parsec::parse_separated_small2_list_expected;

pub(crate) type TemplateSynParametersData = SmallVec<[TemplateSynParameterData; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub struct SynTemplateParameterSyndicateList {
    langle: LaOrLtRegionalToken,
    template_parameters: TemplateSynParametersData,
    commas: CommaRegionalTokens,
    decl_list_result: Result<(), SynNodeDeclError>,
    rangle: RaOrGtRegionalToken,
}

impl SynTemplateParameterSyndicateList {
    pub fn lcurl(&self) -> LaOrLtRegionalToken {
        self.langle
    }

    pub fn syn_template_parameter_obelisks(&self) -> &[TemplateSynParameterData] {
        &self.template_parameters
    }

    pub fn commas(&self) -> &[CommaRegionalToken] {
        self.commas.as_ref()
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for SynTemplateParameterSyndicateList {
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynNodeDeclResult<Option<Self>> {
        let Some(langle) = ctx.try_parse_option::<LaOrLtRegionalToken>()? else {
            return Ok(None);
        };
        let (template_parameters, commas, decl_list_result) = parse_separated_small2_list_expected(
            ctx,
            1,
            OriginalSynNodeDeclError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            template_parameters,
            commas,
            decl_list_result,
            rangle: ctx.try_parse_expected(|regional_token_stream_state| {
                OriginalSynNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                    langle_regional_token_idx: langle.regional_token_idx(),
                    regional_token_stream_state,
                }
            })?,
        }))
    }
}
