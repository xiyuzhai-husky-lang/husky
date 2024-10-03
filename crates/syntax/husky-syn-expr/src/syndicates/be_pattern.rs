use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BePatternSyndicate {
    pattern_root: BeSynPatternRoot,
    variables: CurrentVariableIdxRange,
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
        let Some(pattern_root) = self.try_parse_option::<BeSynPatternRoot>()? else {
            Err(OriginalSynExprError::ExpectedBePattern(state))?
        };
        let symbols = self
            .pattern_region()
            .pattern_variables(pattern_root.syn_pattern_idx());
        let access_start = self.state().next_regional_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_variable)| {
                CurrentVariableEntry::new(
                    self.pattern_region(),
                    access_start,
                    Some(access_end),
                    CurrentVariableData::BeVariable {
                        ident: *ident,
                        pattern_variable_idx: *pattern_variable,
                    },
                )
            })
            .collect::<Vec<_>>();
        let variables = self.define_symbols(symbols, None);
        Ok(BePatternSyndicate {
            pattern_root,
            variables,
        })
    }
}

impl BePatternSyndicate {
    pub fn syn_pattern_root(&self) -> BeSynPatternRoot {
        self.pattern_root
    }

    pub fn variables(&self) -> CurrentVariableIdxRange {
        self.variables
    }
}
