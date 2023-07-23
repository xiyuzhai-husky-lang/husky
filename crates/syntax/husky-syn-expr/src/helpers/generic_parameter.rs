use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct GenericParameterDecl {
    annotated_variance_token: Option<VarianceToken>,
    symbol: CurrentSynSymbolIdx,
    variant: GenericParameterDeclPatternVariant,
}

impl GenericParameterDecl {
    pub fn symbol(&self) -> ArenaIdx<CurrentSynSymbol> {
        self.symbol
    }

    pub fn variant(&self) -> &GenericParameterDeclPatternVariant {
        &self.variant
    }

    pub fn annotated_variance_token(&self) -> Option<VarianceToken> {
        self.annotated_variance_token
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum GenericParameterDeclPatternVariant {
    Type {
        ident_token: IdentToken,
        traits: Option<(ColonToken, SynExprIdx)>,
    },
    Constant {
        const_token: ConstToken,
        ident_token: IdentToken,
        colon_token: ColonToken,
        ty_expr: SynExprIdx,
    },
    Lifetime {
        label_token: LifetimeLabelToken,
    },
    Binding {
        label_token: BindingLabelToken,
    },
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for GenericParameterDecl {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> SynExprResult<Option<Self>> {
        let annotated_variance_token = ctx.try_parse_err_as_none();
        if let Some(ident_token) = ctx.try_parse_option::<IdentToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let parameter_symbol = CurrentSynSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSynSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                        ident_token,
                    },
                },
            );
            let symbols = ctx.define_symbols(
                [parameter_symbol],
                Some(PatternTypeConstraint::ImplicitTypeParameter),
            );
            Ok(Some(GenericParameterDecl {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: GenericParameterDeclPatternVariant::Type {
                    ident_token,
                    traits: if let Some(colon) = ctx.try_parse_option::<ColonToken>()? {
                        Some((
                            colon,
                            ctx.parse_expr_expected2(
                                Some(ExprEnvironment::WithinBracketedParameterList(
                                    Bracket::TemplateAngle,
                                )),
                                ExprRootKind::Traits,
                                OriginalExprError::ExpectedTraits,
                            ),
                        ))
                    } else {
                        None
                    },
                },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<LifetimeLabelToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let symbols = ctx.define_symbols(
                [CurrentSynSymbol::new(
                    ctx.pattern_expr_region(),
                    access_start,
                    None,
                    CurrentSynSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                            label_token,
                        },
                    },
                )],
                Some(PatternTypeConstraint::ImplicitTypeParameter),
            );
            Ok(Some(GenericParameterDecl {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: GenericParameterDeclPatternVariant::Lifetime { label_token },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<BindingLabelToken>()? {
            let symbol = todo!();
            Ok(Some(GenericParameterDecl {
                annotated_variance_token,
                symbol,
                variant: GenericParameterDeclPatternVariant::Binding { label_token },
            }))
        } else if let Some(const_token) = ctx.try_parse_option::<ConstToken>()? {
            let ident_token = ctx.try_parse_expected(OriginalExprError::ExpectedIdent)?;
            let colon_token = ctx.try_parse_expected(OriginalExprError::ExpectedColon)?;
            let ty_expr = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    Bracket::TemplateAngle,
                )),
                ExprRootKind::ConstantImplicitParameterType,
                OriginalExprError::ExpectedConstantImplicitParameterType,
            );
            let access_start = ctx.save_state().next_token_idx();
            let symbol = ctx
                .define_symbols(
                    [CurrentSynSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolVariant::ImplicitParameter {
                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                ident_token,
                                ty_expr_idx: ty_expr,
                            },
                        },
                    )],
                    Some(PatternTypeConstraint::ImplicitTypeParameter),
                )
                .start(); // take start because there is only one symbol to define
            Ok(Some(GenericParameterDecl {
                annotated_variance_token,
                symbol,
                variant: GenericParameterDeclPatternVariant::Constant {
                    const_token,
                    ident_token,
                    colon_token,
                    ty_expr,
                },
            }))
        } else {
            match annotated_variance_token {
                Some(_) => todo!(),
                None => Ok(None),
            }
        }
    }
}
