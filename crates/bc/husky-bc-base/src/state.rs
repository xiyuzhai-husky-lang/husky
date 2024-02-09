use crate::stmt::BcStmtIdx;

pub struct BcState {
    goal: BcStmtIdx,
    stmts: Vec<BcStmtIdx>,
}
