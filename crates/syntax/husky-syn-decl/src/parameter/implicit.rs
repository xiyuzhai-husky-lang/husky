use super::*;
use parsec::parse_separated_small2_list_expected;

pub(crate) type ImplicitParameterDeclPatterns = SmallVec<[GenericParameterDecl; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
pub struct Generics {
    langle: LeftAngleBracketOrLessThanToken,
    generic_parameters: ImplicitParameterDeclPatterns,
    commas: CommaTokens,
    decl_list_result: Result<(), NodeDeclError>,
    rangle: RightAngleBracketToken,
}

impl Generics {
    pub fn lcurl(&self) -> LeftAngleBracketOrLessThanToken {
        self.langle
    }

    pub fn generic_parameters(&self) -> &[GenericParameterDecl] {
        &self.generic_parameters
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
        let Some(langle) = ctx.try_parse_option::< LeftAngleBracketOrLessThanToken>()? else {
            return Ok(None)
        };
        let (generic_parameters, commas, decl_list_result) = parse_separated_small2_list_expected(
            ctx,
            1,
            OriginalNodeDeclError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            generic_parameters,
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
