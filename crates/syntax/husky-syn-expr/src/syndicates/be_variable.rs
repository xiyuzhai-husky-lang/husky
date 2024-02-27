use parsec::HasStreamState;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BePatternSyndicate {
    pattern_expr_root: BeSynPatternExprRoot,
    variables: CurrentSynSymbolIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_be_variables_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<BePatternSyndicate> {
        let state = self.state();
        let Some(pattern_expr_root) = self.try_parse_option::<BeSynPatternExprRoot>()? else {
            Err(OriginalSynExprError::ExpectedBePattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr_root.syn_pattern_expr_idx());
        let access_start = self.state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                CurrentSynSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSynSymbolData::BeVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(BePatternSyndicate {
            pattern_expr_root,
            variables,
        })
    }
}

impl BePatternSyndicate {
    pub fn syn_pattern_root(&self) -> BeSynPatternExprRoot {
        self.pattern_expr_root
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
