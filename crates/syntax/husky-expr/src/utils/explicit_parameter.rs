use super::*;
use parsec::{HasStreamState, TryParseOptionalFromStream};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        dot_dot_dot_token_idx: TokenIdx,
        ident: IdentToken,
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
                        CurrentSymbolVariant::ExplicitParameter {
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
            let variables = ctx.define_symbols(
                variables,
                Some(PatternTypeConstraint::ExplicitParameter {
                    pattern_expr: pattern,
                    ty,
                }),
            );
            todo!("keyed variadics");
            Ok(Some(ExplicitParameterDecl::Regular {
                pattern,
                variables,
                colon,
                ty,
            }))
        } else {
            Ok(None)
        }
    }
}
