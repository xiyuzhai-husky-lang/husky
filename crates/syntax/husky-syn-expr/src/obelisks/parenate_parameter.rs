use super::*;
use either::*;
use parsec::{HasStreamState, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum VariadicVariant {
    Default,
    Vec {
        lbox_token: LboxToken,
        rbox_token: RboxToken,
    },
}

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for VariadicVariant {
    type Error = ExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext<'a, 'b>) -> Result<Self, Self::Error> {
        if let Some(lbox_token) = sp.try_parse_option::<LboxToken>()? {
            if let Some(rbox_token) = sp.try_parse_option::<RboxToken>()? {
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
        pattern: SynPatternExprIdx,
        variables: CurrentSynSymbolIdxRange,
        colon: ColonToken,
        ty: SynExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotToken,
        variadic_variant: VariadicVariant,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierTokenGroup>,
        ident_token: IdentToken,
        variable: CurrentSynSymbolIdx,
        colon: ColonToken,
        ty: SynExprIdx,
    },
    Keyed {
        pattern: SynPatternExprIdx,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierTokenGroup>,
        ident_token: IdentToken,
        variable: CurrentSynSymbolIdx,
        colon: ColonToken,
        ty: SynExprIdx,
        eq_token: EqToken,
        // todo: change this to custom enum
        default: Either<UnderscoreToken, SynExprIdx>,
    },
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for SpecificParameterObelisk {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(pattern_expr_idx) = ctx.parse_pattern_expr(SynPatternExprInfo::Parameter)? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(pattern_expr_idx);
            let access_start = ctx.save_state().next_token_idx();
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
            let colon = ctx.try_parse_expected(OriginalExprError::ExpectedColon)?;
            let ty_expr_idx = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(ObeliskTypeConstraint::ExplicitRegularParameter {
                    pattern_expr_idx,
                    ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<EqToken>()? {
                let SynPatternExpr::Ident {
                    symbol_modifier_keyword_group,
                    ident_token,
                } = ctx.pattern_expr_region()[pattern_expr_idx]
                else {
                    todo!()
                };
                // todo: KeyedWithoutDefault
                let default = if let Some(_) = ctx.try_parse_option::<UnderscoreToken>()? {
                    todo!()
                } else {
                    Right(ctx.parse_expr_expected2(
                        Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                        ExprRootKind::ExplicitParameterDefaultValue { ty_expr_idx },
                        OriginalExprError::ExpectedExplicitParameterDefaultValue,
                    ))
                };
                Ok(Some(SpecificParameterObelisk::Keyed {
                    pattern: pattern_expr_idx,
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
                    pattern: pattern_expr_idx,
                    variables,
                    colon,
                    ty: ty_expr_idx,
                }))
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_option::<DotDotDotToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let variadic_variant = ctx.try_parse()?;
            let symbol_modifier_keyword_group =
                ctx.try_parse_option::<EphemSymbolModifierTokenGroup>()?;
            let ident_token =
                ctx.try_parse_expected::<IdentToken, _>(OriginalExprError::ExpectedIdent)?;
            let variable = CurrentSynSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSynSymbolVariant::ParenateVariadicParameter {
                    ident_token,
                    symbol_modifier_keyword_group,
                },
            );
            let colon = ctx.try_parse_expected(OriginalExprError::ExpectedColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalExprError::ExpectedParameterType,
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
