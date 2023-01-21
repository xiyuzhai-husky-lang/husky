use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterDeclPattern {
    pattern_expr_idx: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ParameterDeclPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(pattern_expr_idx) = ctx.parse_pattern_expr(PatternExprInfo::Parameter)? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_symbol_map(pattern_expr_idx);
            let access_start = ctx.state();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol)| {
                    CurrentSymbol::new(
                        *ident,
                        access_start,
                        None,
                        CurrentSymbolVariant::Parameter {
                            pattern_symbol: *pattern_symbol,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let variables = ctx.define_symbols(variables);
            Ok(Some(ParameterDeclPattern {
                pattern_expr_idx,
                variables,
            }))
        } else {
            Ok(None)
        }
    }
}

impl ParameterDeclPattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}
