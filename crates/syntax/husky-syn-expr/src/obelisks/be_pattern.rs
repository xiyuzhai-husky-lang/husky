use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct BePatternObelisk {
    pattern_expr: SynPatternRoot,
    variables: SynCurrentSymbolIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_be_variables_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<BePatternObelisk> {
        let state = self.save_state();
        let Some(pattern_expr) = self.try_parse_option()? else {
            Err(OriginalSynExprError::ExpectedBePattern(state))?
        };
        let symbols = self
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr);
        let access_start = self.save_state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                SynCurrentSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    SynCurrentSymbolVariant::BeVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(BePatternObelisk {
            pattern_expr,
            variables,
        })
    }
}

impl BePatternObelisk {
    pub fn syn_pattern_root(&self) -> SynPatternRoot {
        self.pattern_expr
    }

    pub fn variables(&self) -> SynCurrentSymbolIdxRange {
        self.variables
    }
}
