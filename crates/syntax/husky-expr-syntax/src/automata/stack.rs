use super::*;

#[derive(Default)]
pub(crate) struct AutomataStack {
    oprs: Vec<OnStackOpr>,
    exprs: Vec<RawExpr>,
}

impl AutomataStack {
    pub(super) fn top_opr(&self) -> Option<&OnStackOpr> {
        self.oprs.last()
    }

    pub(super) fn number_of_exprs(&self) -> usize {
        self.exprs.len()
    }

    pub(super) fn pop_opr(&mut self) -> Option<OnStackOpr> {
        self.oprs.pop()
    }

    pub(super) fn push_expr(&mut self, expr: RawExpr) {
        self.exprs.push(expr)
    }

    pub(super) fn pop_expr(&mut self) -> Option<RawExpr> {
        self.exprs.pop()
    }
}
