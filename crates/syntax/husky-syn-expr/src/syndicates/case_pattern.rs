use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct CasePatternSyndicate {
    syn_pattern_root: CaseSynPatternRoot,
    variables: CurrentVariableIdxRange,
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_case_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<CasePatternSyndicate> {
        let state = self.state();
        let Some(syn_pattern_root) = self.try_parse_option::<CaseSynPatternRoot>()? else {
            Err(OriginalSynExprError::ExpectedCasePattern(state))?
        };
        let symbols = self
            .pattern_region()
            .pattern_variables(syn_pattern_root.syn_pattern_idx());
        let access_start = self.state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_variable)| {
                CurrentVariableEntry::new(
                    self.pattern_region(),
                    access_start,
                    Some(access_end),
                    CurrentVariableData::CaseVariable {
                        ident: *ident,
                        pattern_variable_idx: *pattern_variable,
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
    pub fn syn_pattern_root(&self) -> CaseSynPatternRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> CurrentVariableIdxRange {
        self.variables
    }
}
