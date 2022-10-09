use super::*;

#[derive(Default)]
pub(crate) struct AutomataStack {
    oprs: Vec<OnStackOpr>,
    exprs: Vec<RawExpr>,
}
