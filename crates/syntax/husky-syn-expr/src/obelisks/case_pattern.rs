use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct CasePatternSynObelisk {
    syn_pattern_root: SynPatternRoot,
    variables: SynCurrentSymbolIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_case_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<CasePatternSynObelisk> {
        let state = self.save_state();
        let Some(syn_pattern_root) = self.try_parse_option()? else {
            Err(OriginalSynExprError::ExpectedCasePattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(syn_pattern_root);
        let access_start = self.save_state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                SynCurrentSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    SynCurrentSymbolVariant::CaseVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(CasePatternSynObelisk {
            syn_pattern_root,
            variables,
        })
    }
}

impl CasePatternSynObelisk {
    pub fn syn_pattern_root(&self) -> SynPatternRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> SynCurrentSymbolIdxRange {
        self.variables
    }
}
