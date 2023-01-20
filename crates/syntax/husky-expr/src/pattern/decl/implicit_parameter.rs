use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterDeclPattern {
    symbol: CurrentSymbolIdx,
    variant: ImplicitParameterDeclPatternVariant,
}

impl ImplicitParameterDeclPattern {
    pub fn symbol(&self) -> ArenaIdx<CurrentSymbol> {
        self.symbol
    }

    pub fn variant(&self) -> &ImplicitParameterDeclPatternVariant {
        &self.variant
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplicitParameterDeclPatternVariant {
    Type0 { ident_token: IdentifierToken },
    Constant,
    Lifetime,
    Binding,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDeclPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(ident_token) = ctx.parse::<IdentifierToken>()? {
            let access_start = ctx.state();
            let symbols = ctx.define_symbols([CurrentSymbol::new(
                ident_token.ident(),
                access_start,
                None,
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant: ImplicitParameterVariant::Type { ident_token },
                },
            )]);
            Ok(Some(ImplicitParameterDeclPattern {
                symbol: symbols.start(),
                variant: ImplicitParameterDeclPatternVariant::Type0 { ident_token },
            }))
        } else {
            Ok(None)
        }
    }
}
