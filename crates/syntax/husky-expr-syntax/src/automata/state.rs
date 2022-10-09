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
}
