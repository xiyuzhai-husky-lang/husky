use super::*;
use either::*;
use parsec::{HasStreamState, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum ParenateParameterSyndicate {
    Ordinary {
        syn_pattern_root: ParenateSynPatternExprRoot,
        variables: SynCurrentSymbolIdxRange,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotRegionalToken,
        variadic_variant: SynVariadicParameterVariant,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: SynCurrentSymbolIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Keyed {
        syn_pattern_root: ParenateSynPatternExprRoot,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: SynCurrentSymbolIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
        eq_token: EqRegionalToken,
        // todo: change this to custom enum
        default: Either<UnderscoreRegionalToken, SynExprIdx>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum SynVariadicParameterVariant {
    Default,
    Vec {
        lbox_token: LboxRegionalToken,
        rbox_token: RboxRegionalToken,
    },
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for ParenateParameterSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(syn_pattern_root) = ctx.try_parse_option::<ParenateSynPatternExprRoot>()? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(syn_pattern_root);
            let access_start = ctx.save_state().next_regional_token_idx();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol_idx)| {
                    SynCurrentSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        SynCurrentSymbolVariant::ParenateRegularParameter {
                            ident: *ident,
                            pattern_symbol_idx: *pattern_symbol_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr_idx = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    SynBracket::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(SyndicateTypeConstraint::OrdinaryParenateParameter {
                    syn_pattern_root,
                    ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<EqRegionalToken>()? {
                let SynPatternExpr::Ident {
                    symbol_modifier_tokens: symbol_modifier_keyword_group,
                    ident_token,
                } = ctx.pattern_expr_region()[syn_pattern_root.syn_pattern_expr_idx()]
                else {
                    todo!()
                };
                // todo: KeyedWithoutDefault
                let default = if let Some(_) = ctx.try_parse_option::<UnderscoreRegionalToken>()? {
                    todo!()
                } else {
                    Right(ctx.parse_expr_expected2(
                        Some(ExprEnvironment::WithinBracketedParameterList(
                            SynBracket::Par,
                        )),
                        SynExprRootKind::ExplicitParameterDefaultValue {
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
                Ok(Some(ParenateParameterSyndicate::Ordinary {
                    syn_pattern_root: syn_pattern_root,
                    variables,
                    colon,
                    ty: ty_expr_idx,
                }))
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_option::<DotDotDotRegionalToken>()? {
            let access_start = ctx.save_state().next_regional_token_idx();
            let variadic_variant = ctx.try_parse()?;
            let symbol_modifier_keyword_group =
                ctx.try_parse_option::<EphemSymbolModifierRegionalTokens>()?;
            let ident_token = ctx
                .try_parse_expected::<IdentRegionalToken, _>(OriginalSynExprError::ExpectedIdent)?;
            let variable = SynCurrentSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                SynCurrentSymbolVariant::ParenateVariadicParameter {
                    ident_token,
                    symbol_modifier_keyword_group,
                },
            );
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    SynBracket::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variable = ctx.define_symbol(
                variable,
                Some(SyndicateTypeConstraint::VariadicParenateParameter { ty }),
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
