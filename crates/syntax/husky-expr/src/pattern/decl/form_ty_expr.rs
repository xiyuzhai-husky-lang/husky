use super::*;

/// used for memo, var, const, constexpr
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormTypeExpr {
    expr: ExprIdx,
}

impl FormTypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for FormTypeExpr {
    type Error = ExprError;

    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) =
            ctx.parse_expr_root(Some(ExprEnvironment::TypeBeforeEq), ExprRootKind::VarType)
        {
            Ok(Some(FormTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
