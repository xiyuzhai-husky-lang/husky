use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeColonObelisk {
    expr: SynExprIdx,
}

impl ReturnTypeBeforeColonObelisk {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for ReturnTypeBeforeColonObelisk {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::ReturnType) {
            Ok(Some(ReturnTypeBeforeColonObelisk { expr }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeEqObelisk {
    expr: SynExprIdx,
}

impl ReturnTypeBeforeEqObelisk {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for ReturnTypeBeforeEqObelisk {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) =
            ctx.parse_expr_root(ExprEnvironment::TypeBeforeEq, ExprRootKind::ReturnType)
        {
            Ok(Some(ReturnTypeBeforeEqObelisk { expr }))
        } else {
            Ok(None)
        }
    }
}
