use super::*;

#[derive(Default)]
pub(crate) struct AutomataStack {
    oprs: Vec<OnStackOpr>,
    exprs: Vec<Expr>,
}

impl AutomataStack {
    pub(super) fn number_of_exprs(&self) -> usize {
        self.exprs.len()
    }

    pub(super) fn top_opr(&self) -> Option<&OnStackOpr> {
        self.oprs.last()
    }

    pub(super) fn push_opr(&mut self, opr: OnStackOpr) {
        self.oprs.push(opr)
    }

    pub(super) fn pop_opr(&mut self) -> Option<OnStackOpr> {
        self.oprs.pop()
    }

    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.exprs.last()
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        self.exprs.push(expr)
    }

    pub(super) fn pop_expr(&mut self) -> Option<Expr> {
        self.exprs.pop()
    }

    pub(super) fn topk_exprs(&self, k: usize) -> &[Expr] {
        &self.exprs[(self.exprs.len() - k)..]
    }

    pub(super) fn drain_exprs(&mut self, k: usize) -> Vec<Expr> {
        self.exprs.drain((self.exprs.len() - k)..).collect()
    }
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn expr(&self, idx: usize) -> &Expr {
        &self.stack.exprs[idx]
    }
}
