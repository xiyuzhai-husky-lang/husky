use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LetVariablePattern {
    pattern: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
    colon_token: ExprResult<Option<ColonToken>>,
    ty: Option<ExprIdx>,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_let_variable_pattern(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> ExprResult<LetVariablePattern> {
        let state = self.state();
        if let Some(pattern) = self.parse_pattern_expr(PatternExprInfo::Let)? {
            let symbols = self.pattern_expr_region().pattern_symbol_map(pattern);
            let access_start = self.state();
            let symbols = symbols
                .iter()
                .map(|(ident, pattern_symbol)| {
                    CurrentSymbol::new(
                        *ident,
                        access_start,
                        Some(access_end),
                        CurrentSymbolVariant::LetVariable {
                            pattern_symbol: *pattern_symbol,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon_token = self.parse::<ColonToken>();
            let ty = match colon_token {
                Ok(Some(_)) => self.parse_expr(ExprParseEnvironment::None),
                _ => None,
            };
            let ty_annotation = ty.map(|ty| TypeAnnotation::LetVariables { pattern, ty });
            let variables = self.define_symbols(symbols, todo!());
            Ok(LetVariablePattern {
                pattern,
                variables,
                colon_token,
                ty,
            })
        } else {
            Err(ExprError::ExpectLetVariablePattern(state))
        }
    }
}

impl LetVariablePattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern
    }
}
