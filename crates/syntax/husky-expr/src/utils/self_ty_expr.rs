use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SelfTypeExpr {
    expr: ExprIdx,
}

impl SelfTypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for SelfTypeExpr {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::SelfType) {
            Ok(Some(SelfTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
