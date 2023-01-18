use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LetVariablePattern {
    pattern_expr_idx: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_let_variable_pattern(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> ExprResult<LetVariablePattern> {
        let state = self.state();
        if let Some(pattern_expr_idx) = self.parse_pattern_expr(PatternExprInfo::Let)? {
            let symbols = self
                .pattern_expr_page()
                .pattern_symbol_map(pattern_expr_idx);
            let access_start = self.state();
            let symbols = symbols
                .iter()
                .map(|(ident, pattern_symbol)| {
                    CurrentSymbol::new(
                        *ident,
                        access_start,
                        Some(access_end),
                        CurrentSymbolVariant::LetVariable {
                            pattern_symbol: *pattern_symbol,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let variables = self.define_symbols(symbols);
            Ok(LetVariablePattern {
                pattern_expr_idx,
                variables,
            })
        } else {
            Err(ExprError::ExpectLetVariablePattern(state))
        }
    }
}

impl LetVariablePattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}
