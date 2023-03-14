use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub struct ExplicitParameterDeclList {
    lpar: LeftParenthesisToken,
    parameters: Vec<ExplicitParameterDeclPattern>,
    commas: Vec<CommaToken>,
    decl_list_result: Result<(), DeclExprError>,
    rpar: DeclExprResult<RightParenthesisToken>,
}

impl ExplicitParameterDeclList {
    pub fn parameters<'a>(&'a self) -> DeclExprResultRef<'a, &'a [ExplicitParameterDeclPattern]> {
        self.decl_list_result.as_ref()?;
        self.rpar.as_ref()?;
        Ok(self.parameters.as_ref())
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ExplicitParameterDeclList {
    type Error = DeclExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> DeclExprResult<Option<Self>> {
        let Some(lpar) = ctx.parse::<LeftParenthesisToken>()? else {
        return Ok(None)
    };
        let (parameters, commas, decl_list_result) = parse_separated_list(ctx);
        let rpar = ctx.parse_expected(OriginalDeclExprError::ExpectRightParenthesisInParameterList);
        Ok(Some(Self {
            lpar,
            parameters,
            commas,
            decl_list_result: decl_list_result.map_err(|e| e.into()),
            rpar,
        }))
    }
}
