use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct BeVariableDeclPattern {
    pattern_expr_idx: PatternExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for BeVariableDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        // ad hoc
        if let Some(ident_token) = ctx.parse::<IdentToken>()? {
            Ok(Some(BeVariableDeclPattern {
                pattern_expr_idx: ctx.alloc_pattern_expr(
                    PatternExpr::Ident {
                        ident_token,
                        modifier: PatternModifier::None,
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
