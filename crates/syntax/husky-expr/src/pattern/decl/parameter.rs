use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RegularParameterDeclPattern {
    pattern: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
    colon: ColonToken,
    ty: ExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for RegularParameterDeclPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(pattern) = ctx.parse_pattern_expr(PatternExprInfo::Parameter)? {
            let symbols = ctx.pattern_expr_region().pattern_symbol_map(pattern);
            let access_start = ctx.state();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol_idx)| {
                    CurrentSymbol::new(
                        *ident,
                        access_start,
                        None,
                        CurrentSymbolVariant::RegularParameter {
                            pattern_symbol_idx: *pattern_symbol_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.parse_expected::<ColonToken>()?;
            let Some(ty) = ctx.parse_expr(ExprParseEnvironment::WithinBracket(Bracket::Par)) else {
                todo!()
            };
            let variables = ctx.define_symbols(
                variables,
                Some(PatternTypeConstraint::RegularParameter { pattern, ty }),
            );
            Ok(Some(RegularParameterDeclPattern {
                pattern,
                variables,
                colon,
                ty,
            }))
        } else {
            Ok(None)
        }
    }
}

impl RegularParameterDeclPattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern
    }

    pub fn pattern(&self) -> ArenaIdx<PatternExpr> {
        self.pattern
    }

    pub fn variables(&self) -> ArenaIdxRange<CurrentSymbol> {
        self.variables
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ArenaIdx<Expr> {
        self.ty
    }
}
