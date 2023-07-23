use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct BeVariablesPattern {
    pattern_expr: PatternSynExprIdx,
    variables: CurrentSynSymbolIdxRange,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_be_variables_pattern_expected(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> SynExprResult<BeVariablesPattern> {
        let state = self.save_state();
        let Some(pattern_expr) = self.parse_pattern_expr(
            PatternSynExprInfo::Let
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
                CurrentSynSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSynSymbolVariant::LetVariable {
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
    pub fn pattern_expr(&self) -> PatternSynExprIdx {
        self.pattern_expr
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
