use super::*;
use parsec::{HasStreamState, TryParseOptionalFromStream};

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
        if let Some(lbox_token) = sp.try_parse_optional::<LeftBoxBracketToken>()? {
            if let Some(rbox_token) = sp.try_parse_optional::<RightBoxBracketToken>()? {
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
    KeyedWithoutDefault {
        pattern: PatternExprIdx,
        modifier_keyword_group: Option<PatternSymbolModifierKeywordGroup>,
        ident_token: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
        eq: EqToken,
    },
    KeyedWithDefault {
        pattern: PatternExprIdx,
        modifier_keyword_group: Option<PatternSymbolModifierKeywordGroup>,
        ident_token: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotToken,
        variadic_variant: VariadicVariant,
        modifier_keyword_group: Option<PatternSymbolModifierKeywordGroup>,
        ident_token: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
    },
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for ExplicitParameterDecl {
    type Error = ExprError;

    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(pattern) = ctx.parse_pattern_expr(PatternExprInfo::Parameter)? {
            let symbols = ctx.pattern_expr_region().pattern_expr_symbols(pattern);
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
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(PatternTypeConstraint::ExplicitRegularParameter {
                    pattern_expr: pattern,
                    ty,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_optional::<EqToken>()? {
                let PatternExpr::Ident {
                    modifier_keyword_group,
                    ident_token,
                } = ctx.pattern_expr_region()[pattern] else {
                    todo!()
                };
                // todo: KeyedWithoutDefault
                Ok(Some(ExplicitParameterDecl::KeyedWithDefault {
                    pattern,
                    modifier_keyword_group,
                    ident_token,
                    variable: variables.start(),
                    colon,
                    ty,
                }))
            } else {
                Ok(Some(ExplicitParameterDecl::Regular {
                    pattern,
                    variables,
                    colon,
                    ty,
                }))
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_optional::<DotDotDotToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let variadic_variant = ctx.try_parse()?;
            let modifier_keyword_group = ctx.try_parse_optional()?;
            let ident_token =
                ctx.try_parse_expected::<IdentToken, _>(OriginalExprError::ExpectedIdent)?;
            let variable = CurrentSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSymbolVariant::ExplicitVariadicParameter {
                    ident_token,
                    modifier: match modifier_keyword_group {
                        Some(modifier_keyword_group) => match modifier_keyword_group {
                            PatternSymbolModifierKeywordGroup::Mut(_) => SymbolModifier::Mut,
                            PatternSymbolModifierKeywordGroup::RefMut(_, _) => {
                                SymbolModifier::RefMut
                            }
                        },
                        None => SymbolModifier::Pure,
                    },
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
                modifier_keyword_group,
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
