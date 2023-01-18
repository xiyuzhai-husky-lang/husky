use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BeVariableDeclPattern {
    pattern_expr_idx: PatternExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for BeVariableDeclPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        // ad hoc
        if let Some(ident_token) = ctx.parse::<IdentifierToken>()? {
            Ok(Some(BeVariableDeclPattern {
                pattern_expr_idx: ctx.alloc_pattern_expr(
                    PatternExpr::Identifier {
                        ident_token,
                        liason: PatternLiason::None,
                    },
                    PatternExprInfo::Be,
                ),
            }))
        } else {
            Ok(None)
        }
    }
}

impl BeVariableDeclPattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}
