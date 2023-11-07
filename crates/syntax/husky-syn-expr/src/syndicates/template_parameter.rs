use husky_regional_token::IdentRegionalToken;
use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct TemplateParameterSyndicate {
    annotated_variance_token: Option<VarianceRegionalToken>,
    symbol: CurrentSynSymbolIdx,
    data: TemplateParameterSyndicateData,
}

impl TemplateParameterSyndicate {
    pub fn symbol(&self) -> CurrentSynSymbolIdx {
        self.symbol
    }

    pub fn data(&self) -> &TemplateParameterSyndicateData {
        &self.data
    }

    pub fn annotated_variance_token(&self) -> Option<VarianceRegionalToken> {
        self.annotated_variance_token
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum TemplateParameterSyndicateData {
    Type {
        ident_token: IdentRegionalToken,
        traits: Option<(ColonRegionalToken, SynExprIdx)>,
    },
    Constant {
        const_token: ConstRegionalToken,
        ident_token: IdentRegionalToken,
        colon_token: ColonRegionalToken,
        ty_expr: SynExprIdx,
    },
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Place {
        label_token: PlaceLabelRegionalToken,
    },
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for TemplateParameterSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let syn_attrs = ctx.try_parse()?;
        let annotated_variance_token = ctx.try_parse_err_as_none();
        if let Some(ident_token) = ctx.try_parse_option::<IdentRegionalToken>()? {
            let access_start = ctx.save_state().next_regional_token_idx();
            let parameter_symbol = CurrentSynSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSynSymbolData::TemplateParameter {
                    syn_attrs,
                    annotated_variance_token,
                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                        ident_token,
                    },
                },
            );
            let symbols = ctx.define_symbols(
                [parameter_symbol],
                Some(SyndicateTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateParameterSyndicate {
                // todo: maybe we don't need to put it there, it's redundant
                annotated_variance_token,
                symbol: symbols.start(),
                data: TemplateParameterSyndicateData::Type {
                    ident_token,
                    traits: if let Some(colon) = ctx.try_parse_option::<ColonRegionalToken>()? {
                        Some((
                            colon,
                            ctx.parse_expr_expected2(
                                Some(ExprEnvironment::WithinBracketedParameterList(
                                    SynBracket::TurboFish,
                                )),
                                SynExprRootKind::Traits,
                                OriginalSynExprError::ExpectedTraits,
                            ),
                        ))
                    } else {
                        None
                    },
                },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<LifetimeLabelRegionalToken>()? {
            let access_start = ctx.save_state().next_regional_token_idx();
            let symbols = ctx.define_symbols(
                [CurrentSynSymbol::new(
                    ctx.pattern_expr_region(),
                    access_start,
                    None,
                    CurrentSynSymbolData::TemplateParameter {
                        syn_attrs,
                        annotated_variance_token,
                        template_parameter_variant:
                            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token },
                    },
                )],
                Some(SyndicateTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateParameterSyndicate {
                annotated_variance_token,
                symbol: symbols.start(),
                data: TemplateParameterSyndicateData::Lifetime { label_token },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<PlaceLabelRegionalToken>()? {
            let access_start = ctx.save_state().next_regional_token_idx();
            let symbol = ctx
                .define_symbols(
                    [CurrentSynSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolData::TemplateParameter {
                            syn_attrs,
                            annotated_variance_token,
                            template_parameter_variant:
                                CurrentTemplateParameterSynSymbolVariant::Place { label_token },
                        },
                    )],
                    Some(SyndicateTypeConstraint::TemplateTypeParameter),
                )
                .start();
            Ok(Some(TemplateParameterSyndicate {
                annotated_variance_token,
                symbol,
                data: TemplateParameterSyndicateData::Place { label_token },
            }))
        } else if let Some(const_token) = ctx.try_parse_option::<ConstRegionalToken>()? {
            let ident_token: IdentRegionalToken =
                ctx.try_parse_expected(OriginalSynExprError::ExpectedIdent)?;
            let colon_token = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    SynBracket::TurboFish,
                )),
                SynExprRootKind::ConstantImplicitParameterType,
                OriginalSynExprError::ExpectedConstantImplicitParameterType,
            );
            let access_start = ctx.save_state().next_regional_token_idx();
            let symbol = ctx
                .define_symbols(
                    [CurrentSynSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolData::TemplateParameter {
                            syn_attrs,
                            annotated_variance_token,
                            template_parameter_variant:
                                CurrentTemplateParameterSynSymbolVariant::Constant {
                                    ident_token,
                                    ty_expr_idx: ty_expr,
                                },
                        },
                    )],
                    Some(SyndicateTypeConstraint::TemplateTypeParameter),
                )
                .start(); // take start because there is only one symbol to define
            Ok(Some(TemplateParameterSyndicate {
                annotated_variance_token,
                symbol,
                data: TemplateParameterSyndicateData::Constant {
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
