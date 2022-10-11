use super::*;

#[derive(Default)]
pub(crate) struct AutomataStack {
    oprs: Vec<OnStackOpr>,
    exprs: Vec<RawExpr>,
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

    pub(super) fn top_expr(&self) -> Option<&RawExpr> {
        self.exprs.last()
    }

    pub(super) fn push_expr(&mut self, expr: RawExpr) {
        self.exprs.push(expr)
    }

    pub(super) fn pop_expr(&mut self) -> Option<RawExpr> {
        self.exprs.pop()
    }

    pub(super) fn topk_exprs(&self, k: usize) -> &[RawExpr] {
        &self.exprs[(self.exprs.len() - k)..]
    }

    pub(super) fn drain_exprs(&mut self, k: usize) -> Vec<RawExpr> {
        self.exprs.drain((self.exprs.len() - k)..).collect()
    }
}

impl<'a> Automata<'a> {
    pub(super) fn expr(&self, idx: usize) -> &RawExpr {
        &self.stack.exprs[idx]
    }
}
