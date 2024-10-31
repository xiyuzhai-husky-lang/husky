use super::{expr::VdSynExprData, incomplete_expr::IncompleteVdSynExprData};
use visored_opr::precedence::VdPrecedence;

pub(crate) struct VdSynExprStack {
    incomplete_exprs: Vec<(IncompleteVdSynExprData, VdPrecedence)>,
    complete_expr: Option<VdSynExprData>,
}
