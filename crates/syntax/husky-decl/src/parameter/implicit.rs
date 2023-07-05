use super::*;
use parsec::parse_separated_small2_list_expected;

pub(crate) type ImplicitParameterDeclPatterns = SmallVec<[ImplicitParameterDecl; 2]>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub struct ImplicitParameterDeclList {
    langle: LeftAngleBracketOrLessThanToken,
    implicit_parameters: ImplicitParameterDeclPatterns,
    commas: CommaTokens,
    decl_list_result: Result<(), NodeDeclError>,
    rangle: RightAngleBracketToken,
}

impl ImplicitParameterDeclList {
    pub fn lcurl(&self) -> LeftAngleBracketOrLessThanToken {
        self.langle
    }

    pub fn implicit_parameters(&self) -> &[ImplicitParameterDecl] {
        &self.implicit_parameters
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for ImplicitParameterDeclList {
    type Error = NodeDeclError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> NodeDeclResult<Option<Self>> {
        let Some(langle) = ctx.try_parse_optional::< LeftAngleBracketOrLessThanToken>()? else {
            return Ok(None)
        };
        let (implicit_parameters, commas, decl_list_result) = parse_separated_small2_list_expected(
            ctx,
            1,
            OriginalNodeDeclError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            implicit_parameters,
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
