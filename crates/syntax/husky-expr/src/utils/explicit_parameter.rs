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
        ident: IdentToken,
        variable: CurrentSymbolIdx,
        colon: ColonToken,
        ty: ExprIdx,
        eq: EqToken,
    },
    KeyedWithDefault {
        ident: IdentToken,
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
                Some(ExprEnvironment::WithinBracket(Bracket::Par)),
                ExprRootKind::ExplicitParameterType,
                OriginalExprError::ExpectedParameterType,
            );
            if let Some(eq_token) = ctx.try_parse_optional::<EqToken>()? {
                todo!()
            } else {
                let variables = ctx.define_symbols(
                    variables,
                    Some(PatternTypeConstraint::ExplicitParameter {
                        pattern_expr: pattern,
                        ty,
                    }),
                );
                Ok(Some(ExplicitParameterDecl::Regular {
                    pattern,
                    variables,
                    colon,
                    ty,
                }))
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_optional::<DotDotDotToken>()? {
            let access_start = ctx.save_state().next_token_idx();
            let ident_token =
                ctx.try_parse_expected::<IdentToken, _>(OriginalExprError::ExpectedIdent)?;
            let variable = CurrentSymbol::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentSymbolVariant::ExplicitVariadicParameter { ident_token },
            );
            Ok(Some(ExplicitParameterDecl::Variadic {
                dot_dot_dot_token,
                variadic_variant: ctx.try_parse()?,
                modifier_keyword_group: ctx.try_parse_optional()?,
                ident_token,
                variable: todo!(),
                colon: todo!(),
                ty: todo!(),
            }))
        } else {
            Ok(None)
        }
    }
}
