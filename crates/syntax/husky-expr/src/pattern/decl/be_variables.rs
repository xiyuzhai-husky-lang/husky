use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct BeVariablesPattern {
    pattern_expr: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_be_variables_pattern_expected(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> ExprResult<BeVariablesPattern> {
        let state = self.save_state();
        let Some(pattern_expr) = self.parse_pattern_expr(
            PatternExprInfo::Let
        )? else {
            Err(OriginalExprError::ExpectedBeVariablesPattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr);
        let access_start = self.save_state().next_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                CurrentSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSymbolVariant::LetVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(BeVariablesPattern {
            pattern_expr,
            variables,
        })
    }
}

impl BeVariablesPattern {
    pub fn pattern_expr(&self) -> PatternExprIdx {
        self.pattern_expr
    }

    pub fn variables(&self) -> CurrentSymbolIdxRange {
        self.variables
    }
}
