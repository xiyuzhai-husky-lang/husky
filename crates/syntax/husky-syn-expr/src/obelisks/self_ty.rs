use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SelfTypeObelisk {
    expr: SynExprIdx,
}

impl SelfTypeObelisk {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for SelfTypeObelisk {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::SelfType) {
            Ok(Some(SelfTypeObelisk { expr }))
        } else {
            Ok(None)
        }
    }
}
