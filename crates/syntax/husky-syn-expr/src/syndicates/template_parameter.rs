use super::*;
use crate::syndicates::trais::TraitsSyndicate;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_token_data::delimiter::Delimiter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TemplateSynParameterData {
    annotated_variance_token: Option<VarianceRegionalToken>,
    symbol: CurrentVariableIdx,
    variant: TemplateParameterSyndicateVariant,
}

impl TemplateSynParameterData {
    pub fn symbol(&self) -> CurrentVariableIdx {
        self.symbol
    }

    pub fn variant(&self) -> &TemplateParameterSyndicateVariant {
        &self.variant
    }

    pub fn annotated_variance_token(&self) -> Option<VarianceRegionalToken> {
        self.annotated_variance_token
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TemplateParameterSyndicateVariant {
    Type {
        ident_token: IdentRegionalToken,
        traits: Option<TraitsSyndicate>,
    },
    Compterm {
        termic_token: TermicRegionalToken,
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

impl<'a, 'b> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for TemplateSynParameterData
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> SynExprResult<Option<Self>> {
        let syn_attrs = ctx.try_parse()?;
        let annotated_variance_token = ctx.try_parse_err_as_none();
        if let Some(ident_token) = ctx.try_parse_option::<IdentRegionalToken>()? {
            let access_start = ctx.state().next_regional_token_idx();
            let traits = ctx.try_parse_option::<TraitsSyndicate>()?;
            let parameter_symbol = CurrentVariableEntry::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentVariableData::TemplateParameter {
                    syn_attrs,
                    annotated_variance_token,
                    data: CurrentTemplateVariableData::Type {
                        ident_token,
                        trai_syn_expr_idxs: traits
                            .as_ref()
                            .map(|traits| traits.trai_syn_expr_idxs.clone())
                            .unwrap_or_default(),
                    },
                },
            );
            let symbols = ctx.define_symbols(
                [parameter_symbol],
                Some(SyndicateTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateSynParameterData {
                // todo: maybe we don't need to put it there, it's redundant
                annotated_variance_token,
                symbol: symbols.start(),
                variant: TemplateParameterSyndicateVariant::Type {
                    ident_token,
                    traits,
                },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<LifetimeLabelRegionalToken>()? {
            let access_start = ctx.state().next_regional_token_idx();
            let symbols = ctx.define_symbols(
                [CurrentVariableEntry::new(
                    ctx.pattern_expr_region(),
                    access_start,
                    None,
                    CurrentVariableData::TemplateParameter {
                        syn_attrs,
                        annotated_variance_token,
                        data: CurrentTemplateVariableData::Lifetime { label_token },
                    },
                )],
                Some(SyndicateTypeConstraint::TemplateTypeParameter),
            );
            Ok(Some(TemplateSynParameterData {
                annotated_variance_token,
                symbol: symbols.start(),
                variant: TemplateParameterSyndicateVariant::Lifetime { label_token },
            }))
        } else if let Some(label_token) = ctx.try_parse_option::<PlaceLabelRegionalToken>()? {
            let access_start = ctx.state().next_regional_token_idx();
            let symbol = ctx
                .define_symbols(
                    [CurrentVariableEntry::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentVariableData::TemplateParameter {
                            syn_attrs,
                            annotated_variance_token,
                            data: CurrentTemplateVariableData::Place { label_token },
                        },
                    )],
                    Some(SyndicateTypeConstraint::TemplateTypeParameter),
                )
                .start();
            Ok(Some(TemplateSynParameterData {
                annotated_variance_token,
                symbol,
                variant: TemplateParameterSyndicateVariant::Place { label_token },
            }))
        } else if let Some(termic_token) = ctx.try_parse_option::<TermicRegionalToken>()? {
            let ident_token: IdentRegionalToken =
                ctx.try_parse_expected(OriginalSynExprError::ExpectedIdent)?;
            let colon_token = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinDelimiteredParameterList(
                    Delimiter::TurboFish,
                )),
                SynExprRootKind::ConstantImplicitParameterType,
                OriginalSynExprError::ExpectedConstantImplicitParameterType,
            );
            let access_start = ctx.state().next_regional_token_idx();
            let symbol = ctx
                .define_symbols(
                    [CurrentVariableEntry::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentVariableData::TemplateParameter {
                            syn_attrs,
                            annotated_variance_token,
                            data: CurrentTemplateVariableData::Constant {
                                ident_token,
                                ty_expr_idx: ty_expr,
                            },
                        },
                    )],
                    Some(SyndicateTypeConstraint::TemplateTypeParameter),
                )
                .start(); // take start because there is only one symbol to define
            Ok(Some(TemplateSynParameterData {
                annotated_variance_token,
                symbol,
                variant: TemplateParameterSyndicateVariant::Compterm {
                    termic_token,
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
