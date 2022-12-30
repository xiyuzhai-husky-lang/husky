use super::*;

#[derive(Default, Debug)]
pub(crate) struct AutomataStack {
    oprs: Vec<(StackOpr, Precedence)>,
    exprs: Vec<Expr>,
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn number_of_oprs(&self) -> usize {
        self.stack.oprs.len()
    }

    pub(super) fn number_of_exprs(&self) -> usize {
        self.stack.exprs.len()
    }

    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.stack.exprs.last()
    }

    pub(super) fn finish(mut self) -> ExprIdxRange {
        self.sheet.alloc_expr_batch(self.stack.exprs)
    }

    pub(super) fn topk_exprs(&self, k: usize) -> &[Expr] {
        &self.stack.exprs[(self.stack.exprs.len() - k)..]
    }

    pub(super) fn last_token_in_exprs(&self) -> TokenIdx {
        todo!()
    }
    pub(super) fn expr(&self, idx: usize) -> &Expr {
        &self.stack.exprs[idx]
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        self.stack.exprs.push(expr)
    }

    pub(super) fn push_opr(&mut self, opr: StackOpr) {
        let precedence = opr.precedence();
        self.stack.oprs.push((opr, precedence))
    }

    pub(super) fn top_opr(&self) -> Option<&StackOpr> {
        self.stack.oprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn pop_opr(&mut self) -> Option<StackOpr> {
        self.stack.oprs.pop().map(|(opr, _)| opr)
    }

    pub(super) fn drain_exprs(&mut self, k: usize) -> Vec<Expr> {
        self.stack
            .exprs
            .drain((self.stack.exprs.len() - k)..)
            .collect()
    }
}
