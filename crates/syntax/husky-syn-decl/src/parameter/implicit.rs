use super::*;
use parsec::parse_separated_small2_list_expected;

pub(crate) type ImplicitParameterDeclPatterns = SmallVec<[TemplateParameterObelisk; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub struct Generics {
    langle: LaOrLtRegionalToken,
    template_parameters: ImplicitParameterDeclPatterns,
    commas: CommaRegionalTokens,
    decl_list_result: Result<(), SynNodeDeclError>,
    rangle: RaOrGtRegionalToken,
}

impl Generics {
    pub fn lcurl(&self) -> LaOrLtRegionalToken {
        self.langle
    }

    pub fn template_parameters(&self) -> &[TemplateParameterObelisk] {
        &self.template_parameters
    }

    pub fn commas(&self) -> &[CommaRegionalToken] {
        self.commas.as_ref()
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for Generics {
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
