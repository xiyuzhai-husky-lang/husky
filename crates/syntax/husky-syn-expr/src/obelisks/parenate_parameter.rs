use super::*;
use either::*;
use parsec::{HasStreamState, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum VariadicVariant {
    Default,
    Vec {
        lbox_token: LboxRegionalToken,
        rbox_token: RboxRegionalToken,
    },
}

impl<'a, 'b> TryParseFromStream<SynDeclExprParser<'a>> for VariadicVariant {
    type Error = SynExprError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        if let Some(lbox_token) = sp.try_parse_option::<LboxRegionalToken>()? {
            if let Some(rbox_token) = sp.try_parse_option::<RboxRegionalToken>()? {
                Ok(VariadicVariant::Vec {
                    lbox_token,
                    rbox_token,
                })
            } else {
                todo!()
            }
        } else {
            Ok(VariadicVariant::Default)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum SpecificParameterObelisk {
    Regular {
        syn_pattern_root: SynPatternRoot,
        variables: CurrentSynSymbolIdxRange,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotRegionalToken,
        variadic_variant: VariadicVariant,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentSynSymbolIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Keyed {
        syn_pattern_root: SynPatternRoot,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentSynSymbolIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
        eq_token: RegionalEqToken,
        // todo: change this to custom enum
        default: Either<UnderscoreRegionalToken, SynExprIdx>,
    },
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for SpecificParameterObelisk {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(syn_pattern_root) = ctx.try_parse_option()? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(syn_pattern_root);
            let access_start = ctx.save_state().next_regional_token_idx();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol_idx)| {
                    CurrentSynSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolVariant::ParenateRegularParameter {
                            ident: *ident,
                            pattern_symbol_idx: *pattern_symbol_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr_idx = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(ObeliskTypeConstraint::ExplicitRegularParameter {
                    syn_pattern_root,
                    ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<RegionalEqToken>()? {
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
                        Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                        ExprRootKind::ExplicitParameterDefaultValue { ty_expr_idx },
                        OriginalSynExprError::ExpectedExplicitParameterDefaultValue,
                    ))
                };
                Ok(Some(SpecificParameterObelisk::Keyed {
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
                Ok(Some(SpecificParameterObelisk::Regular {
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
            let variable = CurrentSynSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSynSymbolVariant::ParenateVariadicParameter {
                    ident_token,
                    symbol_modifier_keyword_group,
                },
            );
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variable = ctx.define_symbol(
                variable,
                Some(ObeliskTypeConstraint::ExplicitVariadicParameter { ty }),
            );
            Ok(Some(SpecificParameterObelisk::Variadic {
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
