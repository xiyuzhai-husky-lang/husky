use super::*;

#[salsa::interned(jar = SignatureJar)]
pub struct PropsStructTypeSignature {
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub fields: Vec<PropsStructFieldSignature>,
}

impl PropsStructTypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropsStructFieldSignature {
    ident: IdentifierToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl<'a, 'b> parsec::ParseFrom<ExprParseContext<'a, 'b>> for PropsStructFieldSignature {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(ident) = ctx.parse::<IdentifierToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected()?;
        let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) else { todo!() };
        Ok(Some(PropsStructFieldSignature {
            ident,
            colon,
            ty: expr,
        }))
    }
}
