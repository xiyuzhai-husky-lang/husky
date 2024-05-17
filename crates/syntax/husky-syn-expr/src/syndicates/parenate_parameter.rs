use super::*;
use either::*;
use husky_token_data::delimiter::Delimiter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParenateParameterSyndicate {
    Simple {
        syn_pattern_root: ParenateParameterSynPatternRoot,
        variables: CurrentSynSymbolIdxRange,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotRegionalToken,
        variadic_variant: SynVariadicParameterVariant,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentVariableIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Keyed {
        syn_pattern_root: ParenateParameterSynPatternRoot,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentVariableIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
        eq_token: EqRegionalToken,
        // todo: change this to custom enum
        default: Either<UnderscoreRegionalToken, SynExprIdx>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SynVariadicParameterVariant {
    Default,
    Vec {
        lbox_token: LboxRegionalToken,
        rbox_token: RboxRegionalToken,
    },
}

impl<'a> TryParseOptionFromStream<SynDeclExprParser<'a>> for ParenateParameterSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(syn_pattern_root) = ctx.try_parse_option::<ParenateParameterSynPatternRoot>()? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(syn_pattern_root.syn_pattern_idx());
            let access_start = ctx.state().next_regional_token_idx();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_variable_idx)| {
                    CurrentVariableEntry::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentVariableData::SimpleParenateParameter {
                            ident: *ident,
                            pattern_variable_idx: *pattern_variable_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr_idx = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinDelimiteredParameterList(
                    Delimiter::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(SyndicateTypeConstraint::SimpleParenateParameter {
                    syn_pattern_root,
                    ty: ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<EqRegionalToken>()? {
                let SynPatternData::Ident {
                    symbol_modifier_tokens: symbol_modifier_keyword_group,
                    ident_token,
                } = ctx.pattern_expr_region()[syn_pattern_root.syn_pattern_idx()]
                else {
                    todo!()
                };
                // todo: KeyedWithoutDefault
                let default = if let Some(_) = ctx.try_parse_option::<UnderscoreRegionalToken>()? {
                    todo!()
                } else {
                    Right(ctx.parse_expr_expected2(
                        Some(ExprEnvironment::WithinDelimiteredParameterList(
                            Delimiter::Par,
                        )),
                        SynExprRootKind::ParenateParameterDefaultValue {
                            ty_syn_expr_idx: ty_expr_idx,
                        },
                        OriginalSynExprError::ExpectedExplicitParameterDefaultValue,
                    ))
                };
                Ok(Some(ParenateParameterSyndicate::Keyed {
                    syn_pattern_root,
                    symbol_modifier_keyword_group,
                    ident_token,
                    variable: variables.start(),
                    colon,
                    ty: ty_expr_idx,
                    eq_token,
                    default,
                }))
            } else {
                Ok(Some(ParenateParameterSyndicate::Simple {
                    syn_pattern_root: syn_pattern_root,
                    variables,
                    colon,
                    ty: ty_expr_idx,
                }))
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_option::<DotDotDotRegionalToken>()? {
            let access_start = ctx.state().next_regional_token_idx();
            let variadic_variant = ctx.try_parse()?;
            let symbol_modifier_keyword_group =
                ctx.try_parse_option::<EphemSymbolModifierRegionalTokens>()?;
            let ident_token = ctx
                .try_parse_expected::<IdentRegionalToken, _>(OriginalSynExprError::ExpectedIdent)?;
            let variable = CurrentVariableEntry::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentVariableData::VariadicParenateParameter {
                    ident_token,
                    symbol_modifier_keyword_group,
                },
            );
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinDelimiteredParameterList(
                    Delimiter::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variable = ctx.define_symbol(
                variable,
                Some(SyndicateTypeConstraint::VariadicParenateParameter { ident_token, ty }),
            );
            Ok(Some(ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token,
                variadic_variant,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
            }))
        } else {
            Ok(None)
        }
    }
}

impl<'a, 'b> TryParseFromStream<SynDeclExprParser<'a>> for SynVariadicParameterVariant {
    type Error = SynExprError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        if let Some(lbox_token) = sp.try_parse_option::<LboxRegionalToken>()? {
            if let Some(rbox_token) = sp.try_parse_option::<RboxRegionalToken>()? {
                Ok(SynVariadicParameterVariant::Vec {
                    lbox_token,
                    rbox_token,
                })
            } else {
                todo!()
            }
        } else {
            Ok(SynVariadicParameterVariant::Default)
        }
    }
}
