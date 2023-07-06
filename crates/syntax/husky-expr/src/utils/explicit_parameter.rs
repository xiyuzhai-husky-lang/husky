use super::*;
use either::*;
use parsec::{HasStreamState, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum VariadicVariant {
    Default,
    Vec {
        lbox_token: LeftBoxBracketToken,
        rbox_token: RightBoxBracketToken,
    },
}

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for VariadicVariant {
    type Error = ExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext<'a, 'b>) -> Result<Self, Self::Error> {
        if let Some(lbox_token) = sp.try_parse_option::<LeftBoxBracketToken>()? {
            if let Some(rbox_token) = sp.try_parse_option::<RightBoxBracketToken>()? {
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
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ExplicitParameterDecl {
    Regular {
        pattern: PatternExprIdx,
        variables: CurrentSymbolIdxRange,
        colon: ColonToken,
        ty: ExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotToken,
        variadic_variant: VariadicVariant,
        symbol_modifier_keyword_group: Option<SymbolModifierKeywordGroup>,
        ident_token: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
    },
    Keyed {
        pattern: PatternExprIdx,
        symbol_modifier_keyword_group: Option<SymbolModifierKeywordGroup>,
        ident_token: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
        eq_token: EqToken,
        // todo: change this to custom enum
        default: Either<UnderscoreToken, ExprIdx>,
    },
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for ExplicitParameterDecl {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(pattern_expr_idx) = ctx.parse_pattern_expr(PatternExprInfo::Parameter)? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(pattern_expr_idx);
            let access_start = ctx.save_state().next_token_idx();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol_idx)| {
                    CurrentSymbol::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSymbolVariant::ExplicitRegularParameter {
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
                Some(PatternTypeConstraint::ExplicitRegularParameter {
                    pattern_expr_idx,
                    ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<EqToken>()? {
                let PatternExpr::Ident {
                    symbol_modifier_keyword_group ,
                    ident_token,
                } = ctx.pattern_expr_region()[pattern_expr_idx] else {
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
                Ok(Some(ExplicitParameterDecl::Keyed {
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
                Ok(Some(ExplicitParameterDecl::Regular {
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
                ctx.try_parse_option::<SymbolModifierKeywordGroup>()?;
            let ident_token =
                ctx.try_parse_expected::<IdentToken, _>(OriginalExprError::ExpectedIdent)?;
            let variable = CurrentSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSymbolVariant::ExplicitVariadicParameter {
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
                Some(PatternTypeConstraint::ExplicitVariadicParameter { ty }),
            );
            Ok(Some(ExplicitParameterDecl::Variadic {
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
