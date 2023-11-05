use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeColonSyndicate {
    syn_expr_idx: SynExprIdx,
}

impl ReturnTypeBeforeColonSyndicate {
    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for ReturnTypeBeforeColonSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::ReturnType) {
            Ok(Some(ReturnTypeBeforeColonSyndicate { syn_expr_idx: expr }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeEqSyndicate {
    expr: SynExprIdx,
}

impl ReturnTypeBeforeEqSyndicate {
    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for ReturnTypeBeforeEqSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) =
            ctx.parse_expr_root(ExprEnvironment::TypeBeforeEq, ExprRootKind::ReturnType)
        {
            Ok(Some(ReturnTypeBeforeEqSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
