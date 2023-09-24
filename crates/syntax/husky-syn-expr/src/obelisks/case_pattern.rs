use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct CasePatternObelisk {
    pattern_expr: SynPatternExprIdx,
    variables: CurrentSynSymbolIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_case_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<CasePatternObelisk> {
        let state = self.save_state();
        let Some(pattern_expr) = self.parse_pattern_expr(SynPatternExprEnvironment::Case)? else {
            Err(OriginalSynExprError::ExpectedCasePattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr);
        let access_start = self.save_state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                CurrentSynSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSynSymbolVariant::CaseVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(CasePatternObelisk {
            pattern_expr,
            variables,
        })
    }
}

impl CasePatternObelisk {
    pub fn pattern_expr(&self) -> SynPatternExprIdx {
        self.pattern_expr
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
