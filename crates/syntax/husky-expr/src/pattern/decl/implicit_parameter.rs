use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ImplicitParameterDeclPattern {
    annotated_variance_token: Option<VarianceToken>,
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

    pub fn annotated_variance_token(&self) -> Option<VarianceToken> {
        self.annotated_variance_token
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplicitParameterDeclPatternVariant {
    Type0 { ident_token: IdentToken },
    Constant { ident_token: IdentToken },
    Lifetime { label_token: LifetimeLabelToken },
    Binding { label_token: BindingLabelToken },
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ImplicitParameterDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let annotated_variance_token = ctx.try_parse();
        if let Some(ident_token) = ctx.parse::<IdentToken>()? {
            let access_start = ctx.state();
            let symbols = ctx.define_symbols(
                [CurrentSymbol::new(
                    access_start,
                    None,
                    CurrentSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                            ident_token,
                        },
                    },
                )],
                Some(PatternTypeConstraint::ImplicitTypeParameter),
            );
            Ok(Some(ImplicitParameterDeclPattern {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: ImplicitParameterDeclPatternVariant::Type0 { ident_token },
            }))
        } else if let Some(label_token) = ctx.parse::<LifetimeLabelToken>()? {
            let access_start = ctx.state();
            let symbols = ctx.define_symbols(
                [CurrentSymbol::new(
                    access_start,
                    None,
                    CurrentSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                            label_token,
                        },
                    },
                )],
                Some(PatternTypeConstraint::ImplicitTypeParameter),
            );
            Ok(Some(ImplicitParameterDeclPattern {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: ImplicitParameterDeclPatternVariant::Lifetime { label_token },
            }))
        } else if let Some(label_token) = ctx.parse::<BindingLabelToken>()? {
            let symbol = todo!();
            Ok(Some(ImplicitParameterDeclPattern {
                annotated_variance_token,
                symbol,
                variant: ImplicitParameterDeclPatternVariant::Binding { label_token },
            }))
        } else {
            match annotated_variance_token {
                Some(_) => todo!(),
                None => Ok(None),
            }
        }
    }
}
