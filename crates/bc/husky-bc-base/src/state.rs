use husky_ndp_task::state::IsNdpState;

use crate::stmt::BcStmtIdx;

pub struct BcState {
    goal: BcStmtIdx,
    stmts: Vec<BcStmtIdx>,
}

impl IsNdpState for BcState {}
