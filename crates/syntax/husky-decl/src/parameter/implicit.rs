use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub struct ImplicitParameterDecl {
    pattern: ImplicitParameterDeclPattern,
    traits: Option<(ColonToken, Option<ExprIdx>)>,
}

impl ImplicitParameterDecl {
    pub fn pattern(&self) -> &ImplicitParameterDeclPattern {
        &self.pattern
    }

    pub fn traits(&self) -> Option<(ColonToken, Option<ExprIdx>)> {
        self.traits
    }
}

impl<'a, 'b> ParseFromStream<ExprParseContext<'a, 'b>> for ImplicitParameterDecl {
    type Error = DeclExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> DeclExprResult<Option<Self>> {
        let Some(pattern) = ctx.parse::<ImplicitParameterDeclPattern>()? else {
            return Ok(None)
        };

        Ok(Some(Self {
            pattern,
            traits: if let Some(colon) = ctx.parse::<ColonToken>()? {
                Some((
                    colon,
                    ctx.parse_expr(ExprEnvironment::WithinBracket(Bracket::TemplateAngle)),
                ))
            } else {
                None
            },
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub struct ImplicitParameterDeclList {
    langle: LeftAngleBracketOrLessThanToken,
    implicit_parameters: Vec<ImplicitParameterDecl>,
    commas: Vec<CommaToken>,
    decl_list_result: Result<(), DeclExprError>,
    rangle: DeclExprResult<RightAngleBracketToken>,
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

impl<'a, 'b> ParseFromStream<ExprParseContext<'a, 'b>> for ImplicitParameterDeclList {
    type Error = DeclExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> DeclExprResult<Option<Self>> {
        let Some(langle) = ctx.parse::< LeftAngleBracketOrLessThanToken>()? else {
            return Ok(None)
        };
        let (decls, commas, decl_list_result) = parse_separated_list_expected(
            ctx,
            1,
            OriginalDeclExprError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            implicit_parameters: decls,
            commas,
            decl_list_result,
            rangle: ctx.parse_expected(|token_stream_state| {
                OriginalDeclExprError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                    langle_token_idx: langle.token_idx(),
                    token_stream_state,
                }
            }),
        }))
    }
}
