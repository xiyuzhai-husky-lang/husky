use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct TemplateParameterObelisk {
    annotated_variance_token: Option<VarianceToken>,
    symbol: CurrentSynSymbolIdx,
    variant: TemplateParameterDeclPatternVariant,
}

impl TemplateParameterObelisk {
    pub fn symbol(&self) -> ArenaIdx<CurrentSynSymbol> {
        self.symbol
    }

    pub fn variant(&self) -> &TemplateParameterDeclPatternVariant {
        &self.variant
    }

    pub fn annotated_variance_token(&self) -> Option<VarianceToken> {
        self.annotated_variance_token
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum TemplateParameterDeclPatternVariant {
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
        label_token: LifetimeToken,
    },
    Binding {
        label_token: BindingLabelToken,
    },
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for TemplateParameterObelisk {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let syn_attrs = ctx.try_parse()?;
        let annotated_variance_token = ctx.try_parse_err_as_none();
        if let Some(ident_token) = ctx.try_parse_option::<IdentToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let parameter_symbol = CurrentSynSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSynSymbolVariant::TemplateParameter {
                    syn_attrs,
                    annotated_variance_token,
                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                        ident_token,
                    },
                },
            );
            let symbols = ctx.define_symbols(
                [parameter_symbol],
                Some(ObeliskTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateParameterObelisk {
                // todo: maybe we don't need to put it there, it's redundant
                annotated_variance_token,
                symbol: symbols.start(),
                variant: TemplateParameterDeclPatternVariant::Type {
                    ident_token,
                    traits: if let Some(colon) = ctx.try_parse_option::<ColonToken>()? {
                        Some((
                            colon,
                            ctx.parse_expr_expected2(
                                Some(ExprEnvironment::WithinBracketedParameterList(
                                    Bracket::TemplateAngle,
                                )),
                                ExprRootKind::Traits,
                                OriginalSynExprError::ExpectedTraits,
                            ),
                        ))
                    } else {
                        None
                    },
                },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<LifetimeToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let symbols = ctx.define_symbols(
                [CurrentSynSymbol::new(
                    ctx.pattern_expr_region(),
                    access_start,
                    None,
                    CurrentSynSymbolVariant::TemplateParameter {
                        syn_attrs,
                        annotated_variance_token,
                        template_parameter_variant:
                            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token },
                    },
                )],
                Some(ObeliskTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateParameterObelisk {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: TemplateParameterDeclPatternVariant::Lifetime { label_token },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<BindingLabelToken>()? {
            let symbol = todo!();
            Ok(Some(TemplateParameterObelisk {
                annotated_variance_token,
                symbol,
                variant: TemplateParameterDeclPatternVariant::Binding { label_token },
            }))
        } else if let Some(const_token) = ctx.try_parse_option::<ConstToken>()? {
            let ident_token = ctx.try_parse_expected(OriginalSynExprError::ExpectedIdent)?;
            let colon_token = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    Bracket::TemplateAngle,
                )),
                ExprRootKind::ConstantImplicitParameterType,
                OriginalSynExprError::ExpectedConstantImplicitParameterType,
            );
            let access_start = ctx.save_state().next_token_idx();
            let syn_attrs = ctx.try_parse()?;
            let symbol = ctx
                .define_symbols(
                    [CurrentSynSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolVariant::TemplateParameter {
                            syn_attrs,
                            annotated_variance_token,
                            template_parameter_variant:
                                CurrentTemplateParameterSynSymbolVariant::Constant {
                                    ident_token,
                                    ty_expr_idx: ty_expr,
                                },
                        },
                    )],
                    Some(ObeliskTypeConstraint::TemplateTypeParameter),
                )
                .start(); // take start because there is only one symbol to define
            Ok(Some(TemplateParameterObelisk {
                annotated_variance_token,
                symbol,
                variant: TemplateParameterDeclPatternVariant::Constant {
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
