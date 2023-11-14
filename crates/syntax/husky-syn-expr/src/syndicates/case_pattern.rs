use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct CasePatternSyndicate {
    syn_pattern_root: CaseSynPatternExprRoot,
    variables: CurrentSynSymbolIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_case_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<CasePatternSyndicate> {
        let state = self.save_state();
        let Some(syn_pattern_root) = self.try_parse_option::<CaseSynPatternExprRoot>()? else {
            Err(OriginalSynExprError::ExpectedCasePattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(syn_pattern_root.syn_pattern_expr_idx());
        let access_start = self.save_state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                CurrentSynSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSynSymbolData::CaseVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(CasePatternSyndicate {
            syn_pattern_root,
            variables,
        })
    }
}

impl CasePatternSyndicate {
    pub fn syn_pattern_root(&self) -> CaseSynPatternExprRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
