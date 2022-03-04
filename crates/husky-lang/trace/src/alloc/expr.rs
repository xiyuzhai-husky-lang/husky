use semantics::Expr;
use text::Text;

use super::*;
use crate::*;

impl TraceAllocator {
    pub fn new_expr_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        expr: Arc<Expr>,
        value: TraceStackValue,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceKind::Expr { expr, value },
            text,
        )
    }

    pub(super) fn expr_tokens(
        &self,
        expr: &Expr,
        opt_value: Option<&TraceStackValue>,
    ) -> Vec<TokenProps> {
        match expr.kind {
            semantics::ExprKind::Variable(_) => todo!(),
            semantics::ExprKind::Scope { scope, compiled } => todo!(),
            semantics::ExprKind::Literal(value) => vec![literal!(value)],
            semantics::ExprKind::Bracketed(_) => todo!(),
            semantics::ExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => {
                let mut tokens = vec![];
                match opn {
                    semantics::Opn::Binary { opr, this, kind } => {
                        tokens.extend(self.expr_tokens(&opds[0], None));
                        tokens.push(special!(opr.spaced_code()));
                        tokens.extend(self.expr_tokens(&opds[1], None));
                    }
                    semantics::Opn::Prefix(_) => todo!(),
                    semantics::Opn::Suffix(_) => todo!(),
                    semantics::Opn::RoutineCall(_) => todo!(),
                    semantics::Opn::PattCall => todo!(),
                    semantics::Opn::MembVarAccess => todo!(),
                    semantics::Opn::MembFuncCall(_) => todo!(),
                    semantics::Opn::ElementAccess => todo!(),
                }
                if let Some(value) = opt_value {
                    tokens.push(fade!(" = "));
                    tokens.push(value.clone().into())
                }
                tokens
            }
            semantics::ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
