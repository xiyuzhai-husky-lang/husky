use super::*;
use parsec::parse_separated_small2_list_expected;

pub(crate) type ImplicitParameterDeclPatterns = SmallVec<[TemplateParameterDecl; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDeclDb)]
pub struct Generics {
    langle: LaOrLtToken,
    template_parameters: ImplicitParameterDeclPatterns,
    commas: CommaTokens,
    decl_list_result: Result<(), NodeDeclError>,
    rangle: RaOrGtToken,
}

impl Generics {
    pub fn lcurl(&self) -> LaOrLtToken {
        self.langle
    }

    pub fn template_parameters(&self) -> &[TemplateParameterDecl] {
        &self.template_parameters
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for Generics {
    type Error = NodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> NodeDeclResult<Option<Self>> {
        let Some(langle) = ctx.try_parse_option::<LaOrLtToken>()? else {
            return Ok(None);
        };
        let (template_parameters, commas, decl_list_result) = parse_separated_small2_list_expected(
            ctx,
            1,
            OriginalNodeDeclError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            template_parameters,
            commas,
            decl_list_result,
            rangle: ctx.try_parse_expected(|token_stream_state| {
                OriginalNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                    langle_token_idx: langle.token_idx(),
                    token_stream_state,
                }
            })?,
        }))
    }
}
