use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct LetVariableObelisk {
    pattern_expr_idx: SynPatternExprIdx,
    variables: CurrentSynSymbolIdxRange,
    colon_token: SynExprResult<Option<ColonRegionalToken>>,
    ty: Option<SynExprIdx>,
}

impl<'a, 'b> SynDefnExprParser<'a, 'b> {
    pub(crate) fn parse_let_variables_pattern_expected(
        &mut self,
        access_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<LetVariableObelisk> {
        let state = self.save_state();
        let Some(pattern) = self.parse_pattern_expr(SynPatternExprInfo::Let)? else {
            Err(OriginalSynExprError::ExpectedLetVariableDecls(state))?
        };
        let symbols = self.pattern_expr_region().pattern_expr_symbols(pattern);
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
        let colon_token = self
            .try_parse_option::<ColonRegionalToken>()
            .map_err(|e| e.into());
        let ty = match colon_token {
            Ok(Some(_)) => Some(self.parse_expr_expected2(
                Some(ExprEnvironment::TypeBeforeEq),
                ExprRootKind::LetStmtType,
                OriginalSynExprError::ExpectedLetVariablesType,
            )),
            _ => None,
        };
        let ty_constraint = ty.map(|ty| ObeliskTypeConstraint::LetVariables { pattern, ty });
        let variables = self.define_symbols(symbols, ty_constraint);
        Ok(LetVariableObelisk {
            pattern_expr_idx: pattern,
            variables,
            colon_token,
            ty,
        })
    }
}

impl LetVariableObelisk {
    pub fn pattern_expr_idx(&self) -> SynPatternExprIdx {
        self.pattern_expr_idx
    }

    pub fn ty(&self) -> Option<SynExprIdx> {
        self.ty
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
